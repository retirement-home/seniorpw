use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::error::Error;

use interprocess::local_socket::{LocalSocketListener, LocalSocketStream, NameTypeSupport};

fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
     match conn {
         Ok(c) => Some(c),
         Err(e) => {
             eprintln!("Incoming connection failed: {e}");
             None
         }
     }
}

fn print_conn(conn: &mut interprocess::local_socket::LocalSocketStream, s: &str) -> std::io::Result<()> {
    //print!("{}", s);
    write!(conn, "{}", s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let name = {
         use NameTypeSupport::*;
         match NameTypeSupport::query() {
             OnlyPaths => "/tmp/senior-agent.sock",
             OnlyNamespaced | Both => "@senior-agent.sock",
         }
    };

    let listener = match LocalSocketListener::bind(name) {
         Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
             eprintln!(
                 "\
    Error: could not start server because the socket file is occupied. Please check if {name} is in \
    use by another process and try again."
             );
             return Err(e.into());
         }
         x => x?,
    };

    let mut passphrases = HashMap::<String, String>::new();
    let mut buffer = String::with_capacity(1024);

    println!("Ready for connections!");

    for conn in listener.incoming().filter_map(handle_error) {
        buffer.clear();
        let mut conn = BufReader::new(conn);

        match conn.read_line(&mut buffer) {
            Ok(0) => { println!("Read EOF. Closing."); break; },
            Err(e) => { eprintln!("Error: {}", e); continue; },
            Ok(_) => {},
        }

        // Remove trailing newline
        buffer.pop();

        let mut conn = conn.get_mut();
        match &buffer[0..1] {
            "r" => { // read
                let key = &buffer[2..];
                match passphrases.contains_key(key) {
                    true => print_conn(&mut conn, &format!("o: {}\n", &passphrases[key]))?,
                    false => print_conn(&mut conn, &format!("e: Key {} is not present!\n", key))?,
                }
            },
            "w" => { // write
                // the first space determines the split between key and passphrase
                // spaces in the key must be escaped with a backslash
                let mut prev_char_was_backslash = false;
                let mut separator_index = 0;
                for (i, c) in buffer[2..].char_indices() {
                    if c == ' ' && !prev_char_was_backslash {
                        separator_index = i;
                        break
                    } else if c == '\\' {
                        prev_char_was_backslash = true;
                    } else {
                        prev_char_was_backslash = false;
                    }
                }
                let key = buffer[2..(separator_index+2)].to_owned();
                let pass = buffer[(separator_index+3)..].to_owned();
                //println!("Writing passphrase {} for key {}.", &pass, &key);
                passphrases.insert(key, pass);
            },
            _ => { print_conn(&mut conn, "e: Command not implemented!")?; continue; },
        }
    }
    Ok(())
}

