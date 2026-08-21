use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::iter::Enumerate;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use age::secrecy::SecretString;
use age::{self, ssh};
use walkdir::WalkDir;

// resolve symlinks even if the end of the path does not exist
pub fn canonicalise(path: &Path) -> std::io::Result<PathBuf> {
    fn canonicalise_helper(path: &Path) -> std::io::Result<PathBuf> {
        if path.exists() {
            path.canonicalize()
        } else {
            let parent = canonicalise_helper(path.parent().unwrap())?;
            match path.file_name() {
                Some(filename) => Ok(parent.join(filename)),
                None => Ok(parent.parent().unwrap().to_path_buf()),
            }
        }
    }

    // After one run paths like "nonexistdir/../existsymlink/existfile" are simplified to
    // "existsymlink/existfile". A second run will then resolve all existing symlinks.
    canonicalise_helper(&canonicalise_helper(path)?)
}

pub fn age_identity_from_keyfile_content(
    keyfile_content: &str,
) -> Result<age::x25519::Identity, Box<dyn Error>> {
    let identity_str = keyfile_content
        .lines()
        .find(|&l| l.starts_with("AGE-SECRET-KEY"))
        .ok_or("No identity in keyfile content!")?;
    Ok(age::x25519::Identity::from_str(identity_str)?)
}

pub fn new_passphrase_identity(passphrase: &str) -> age::scrypt::Identity {
    let mut identity = age::scrypt::Identity::new(SecretString::from(passphrase));
    identity.set_max_work_factor(32);
    identity
}

pub fn recipient_from_str(line: &str) -> Result<Box<dyn age::Recipient>, Box<dyn Error>> {
    if line.starts_with("ssh-") {
        ssh::Recipient::from_str(line)
            .map(|r| Box::new(r) as Box<dyn age::Recipient>)
            .map_err(|e| format!("{e:?}").into())
    } else {
        age::x25519::Recipient::from_str(line)
            .map(|r| Box::new(r) as Box<dyn age::Recipient>)
            .map_err(|e| e.into())
    }
}

pub struct RecipientStrIter {
    walkdir: walkdir::IntoIter,
    line_iter: Option<Enumerate<io::Lines<BufReader<File>>>>,
    cur_file: PathBuf,
}

impl RecipientStrIter {
    pub fn new(recipients_dir: &Path) -> Self {
        RecipientStrIter {
            walkdir: WalkDir::new(recipients_dir).into_iter(),
            line_iter: None,
            cur_file: PathBuf::new(),
        }
    }
}

impl Iterator for RecipientStrIter {
    // returns a tuple: (the line with the recipient's public key, <path>:<linenumber>)
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.line_iter.is_none() {
                self.cur_file = loop {
                    let entry = self
                        .walkdir
                        .next()?
                        .expect("Cannot unwrap DirEntry for recipients!")
                        .into_path();
                    if !entry.is_file() {
                        continue;
                    }
                    break entry;
                };
                let reader =
                    BufReader::new(File::open(&self.cur_file).unwrap_or_else(|_| {
                        panic!("Cannot open file {}!", self.cur_file.display())
                    }));
                self.line_iter = Some(reader.lines().enumerate());
            }
            let next_line = loop {
                match self.line_iter.as_mut().unwrap().next() {
                    None => break None,
                    Some((i, line)) => {
                        let filepos = format!("{}:{}", self.cur_file.display(), i + 1);
                        let line = line.unwrap_or_else(|_| panic!("Cannot read line {}!", filepos));
                        if line.trim_start().starts_with('#') || line.trim().is_empty() {
                            continue;
                        }
                        break Some((line, filepos));
                    }
                }
            };
            if next_line.is_none() {
                self.line_iter = None;
                continue;
            }
            break next_line;
        }
    }
}

// returns Some("<filepath>:<linenumber>") if pubkey is already present
pub fn find_pubkey_in_recipients(recipients_dir: &Path, pubkey: &str) -> Option<String> {
    // removes the comment from ssh-keys
    // ssh keys look like this:
    // ssh-<ed25519|rsa> <the-actual-key> <comment>
    // To check if a key is already present we want to ignore the comment line
    fn public_key_without_comment(public_key: &str) -> &str {
        if public_key.starts_with("ssh-") {
            let mut space_indices = public_key.match_indices(' ').take(2);
            // there should always be at least one space in an ssh public key
            assert!(
                space_indices.next().is_some(),
                "There is no space in this ssh key! {public_key}"
            );
            match space_indices.next() {
                // no comment => return entire string
                None => public_key,
                // there is a comment => return string up to the comment
                Some((i, _)) => &public_key[0..i],
            }
        } else {
            public_key
        }
    }

    let pubkey_without_comment = public_key_without_comment(pubkey);

    for (recipient_without_comment, filepos) in RecipientStrIter::new(recipients_dir)
        .map(|(pubkey, filepos)| (public_key_without_comment(&pubkey).to_owned(), filepos))
    {
        if recipient_without_comment == pubkey_without_comment {
            return Some(filepos);
        }
    }
    None
}

// encrypts the contents of source into target_file
pub fn encrypt_password(
    recipients: &[Box<dyn age::Recipient>],
    mut source: impl Read,
    target_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let encryptor = age::Encryptor::with_recipients(recipients.iter().map(|i| i.as_ref()))?;
    let mut writer = encryptor.wrap_output(File::create(target_file)?)?;
    let mut content = vec![];
    source.read_to_end(&mut content)?;
    writer.write_all(&content)?;
    writer.finish()?;
    Ok(())
}

// also removes parent directories until an error is raised
pub fn removedirs(path: &Path) -> io::Result<()> {
    let mut path_buf = path.to_path_buf();
    while path_buf.is_dir() {
        match fs::remove_dir(&path_buf) {
            Ok(()) => {
                if let Some(parent) = path_buf.parent() {
                    path_buf = parent.to_path_buf();
                } else {
                    break;
                }
            }
            Err(err) if err.raw_os_error() == Some(39) => break,
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
