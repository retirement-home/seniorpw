use std::path::PathBuf;
use std::{env, fs};
use std::process::Command;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;

use clap::{Parser, Subcommand};
use which::which;
use tempdir::TempDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// alias for the store; default: "main", or the only existing one
    #[arg(short, long)]
    store: Option<String>,

    /// the age backend to use
    #[arg(long)]
    age: Option<String>,

    /// the command to run; "show" if omitted
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// initialises a new store
    Init {
        /// path of the private key used for decrypting; will be generated if none is supplied
        #[arg(long = "identity")]
        identity: Option<String>,

        /// alias for recipient; defaults to the username
        #[arg(long = "recipient-alias")]
        recipient_alias: Option<String>,
    },

    /// clones a store from a git repository
    Clone {
        /// address of the remote git repository
        #[arg(index = 1)]
        address: String,
    },

    /// edit/create a password
    Edit {
        /// name of the password file
        #[arg(index = 1)]
        name: String,
    },

    /// show the password
    Show {
        /// show only this key
        #[arg(short, long)]
        key: Option<String>,

        /// name of the password file
        #[arg(index = 1)]
        name: String,
    },

    /// show the one-time password
    Otp {
        /// name of the password file
        #[arg(index = 1)]
        name: String,
    },

    /// change to the store's directory
    Cd {},

    /// show the store's directory path
    PrintDir {},

    /// git pull and push
    Sync {},

    /// add recipient
    AddRecipient {
        /// public key of the new recipient; cleartext or path
        #[arg(index = 1)]
        public_key: String
    },

    /// request recipient
    RequestRecipient {
        /// add your key to the requested recipients; cleartext or path
        #[arg(index = 1)]
        public_key: String
    },
}

fn find_age_backend() -> String {
    let known_backends = ["rage", "age"];
    for backend in known_backends {
        if let Ok(_) = which(backend) {
            return String::from(backend);
        }
    }
    panic!("Could not find an age backend!");
}

fn init(cli: &Cli, senior_dir: PathBuf, identity: Option<String>, mut recipient_alias: Option<String>) {
    let store_dir = senior_dir.join(cli.store.as_ref().unwrap());
    assert!(!store_dir.exists(), "The directory of the store exists already");

    // set up default values
    if recipient_alias == None {
        recipient_alias = Some(env::var_os("USER").expect("Could not get the username").into_string().unwrap());
    }

    let tmp_dir = TempDir::new("senior-keygen").expect("Could not create temporary directory");

    // TODO: Support password protected identities
    let (identity_file_src, recipient) = match &identity {
        Some(path_string) => {
            let pathbuf = PathBuf::from(path_string);
            assert!(pathbuf.is_file(), "Supplied identity path is not a file");
            let content = fs::read_to_string(&pathbuf).expect("Could not read identity file");
            let pubkey = if content.contains("SSH PRIVATE KEY") {
                let command = Command::new("ssh-keygen").args(["-y", "-f", &path_string]).output().expect("Could not run ssh-keygen to get public key");
                String::from_utf8(command.stdout).expect("UTF-8 conversion error")
            } else {
                match content.find("public key: ") {
                    Some(start_index) => {
                        let substring = &content[(start_index + "public key: ".len())..];
                        match substring.find("\n") {
                            Some(end_index) => String::from(&content[(start_index + "public key: ".len())..end_index]),
                            None => String::from(substring),
                        }
                    },
                    None => panic!("Cannot read public key from identity file"),
                }
            };
            (pathbuf, pubkey)
        },
        None => {
            let identity_file = tmp_dir.path().join("identity");
            let mut age_keygen = cli.age.as_ref().unwrap().clone();
            age_keygen.push_str("-keygen");
            let command = Command::new(OsString::from(age_keygen)).args(["-o", identity_file.to_str().unwrap()]).output().expect("Could not generate key-pair");
            let output = String::from_utf8_lossy(&command.stderr);
            (identity_file, String::from(&output["Public key: ".len()..]))
        },
    };

    let recipients_dir = store_dir.join(".recipients");
    let recipients_main = recipients_dir.join("main.txt");
    let recipients_request_dir = store_dir.join(".recipients-request");
    let gitignore = store_dir.join(".gitignore");
    let identity_file = store_dir.join(".identity.txt");
    // TODO: .gitattributes file

    fs::create_dir_all(recipients_dir).expect("Could not create .recipients directory");
    fs::create_dir_all(recipients_request_dir).expect("Could not create .recipients-request directory");
    fs::copy(identity_file_src, identity_file).expect("Could not copy .identity file");
    let mut gitignore_file = File::create(gitignore).expect("Could not create gitignore file");
    gitignore_file.write_all(b"/.identity.*\n").expect("Could not write gitignore file");
    let mut recipients_main_file = File::create(recipients_main).expect("Could not create recipients main file");
    write!(recipients_main_file, "# {}\n{}\n", recipient_alias.unwrap(), recipient).expect("Could not write recipients main file");
}

fn main() {
    let mut cli = Cli::parse();

    let senior_dir = match env::var_os("XDG_DATA_HOME") {
        Some(val) => PathBuf::from(val),
        None => PathBuf::from(env::var_os("HOME").unwrap()).join(".local/share"),
    }.join("senior/");

    if cli.store == None {
        cli.store = Some(if senior_dir.is_dir() {
            let mut entries = senior_dir.read_dir().expect("Could not read the senior directory").filter(|entry| entry.as_ref().unwrap().file_type().unwrap().is_dir());
            match (entries.next(), entries.next()) {
                (Some(entry), None) => entry.unwrap().file_name().into_string().unwrap(),
                _ => String::from("main"),
            }
        } else {
            String::from("main")
        });
    }

    if cli.age == None {
        cli.age = Some(find_age_backend());
    }

    match &cli.command {
        Commands::Init { identity, recipient_alias, } => init(&cli, senior_dir, identity.clone(), recipient_alias.clone()),
        _ => panic!("Command not yet implemented"),
    }
}
