use std::ffi::OsString;

use clap::{Parser, Subcommand, builder::ValueHint};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Alias for the store; default: "main", or the only existing one,
    ///                      or for `senior clone` the name of the repository
    #[arg(short, long)]
    pub store: Option<OsString>,

    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliCommand {
    /// Initialises a new store
    Init {
        /// Path of the identity used for decrypting; default: generate a new one
        #[arg(short, long, value_name = "FILE", value_hint = ValueHint::AnyPath)]
        identity: Option<String>,

        /// Alias for recipient; default: your username
        #[arg(short = 'a', long = "recipient-alias", value_name = "USERNAME")]
        recipient_alias: Option<String>,
    },

    /// Clones a store from a git repository
    #[command(name = "clone")]
    GitClone {
        /// Path of the identity used for decrypting; default: generate a new one
        #[arg(short, long, value_name = "FILE", value_hint = ValueHint::AnyPath)]
        identity: Option<String>,

        /// Address of the remote git repository
        #[arg(index = 1, value_hint = ValueHint::Url)]
        address: String,
    },

    /// Edit/create a password
    Edit {
        /// Name of the password file
        #[arg(index = 1, value_hint = ValueHint::AnyPath)]
        name: String,
    },

    /// Show the password
    Show {
        /// Show only this key;
        /// "password" shows the first line;
        /// "otp" generates the one-time password
        #[arg(short, long, value_name = "otp|login|email|...")]
        key: Option<String>,

        /// Add the value to the clipboard
        #[arg(short, long)]
        clip: bool,

        /// Name of the password file
        #[arg(index = 1, default_value_t = String::from(""), value_hint = ValueHint::FilePath)]
        name: String,
    },

    /// Move a password
    Mv {
        /// Old name of the password file or directory
        #[arg(index = 1, value_hint = ValueHint::AnyPath)]
        old_name: String,

        /// New name of the password file or directory
        #[arg(index = 2, value_hint = ValueHint::AnyPath)]
        new_name: String,
    },

    /// Remove a password
    Rm {
        /// For directories
        #[arg(short, long)]
        recursive: bool,

        /// Name of the password file or directory
        #[arg(index = 1, value_hint = ValueHint::AnyPath)]
        name: String,
    },

    /// Show the directory of the store
    PrintDir,

    /// Run git commands in the specified store
    Git {
        #[arg(allow_hyphen_values = true, trailing_var_arg = true, value_hint = ValueHint::CommandWithArguments)]
        args: Vec<String>,
    },

    /// Add recipient
    AddRecipient {
        /// Public key of the new recipient
        #[arg(index = 1, value_name = "PUBLIC KEY")]
        public_key: String,

        /// Name of the new recipient
        #[arg(index = 2)]
        alias: String,
    },

    /// Reencrypt the entire store
    Reencrypt,
}

