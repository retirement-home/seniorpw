use clap::{Parser, Subcommand, builder::ValueHint};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
/// A password manager, inspired by password-store, using age for encryption
pub struct Cli {
    /// Name of the store; default: "main", or the only existing one,
    /// or for `senior clone` the name of the repository
    #[arg(short, long, value_hint = ValueHint::AnyPath)]
    pub store: Vec<String>,

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

        /// Your recipient name; default: username@hostname
        #[arg(short = 'a', long = "recipient-alias", value_name = "USER@HOST")]
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
        /// Name of the password
        #[arg(index = 1, value_hint = ValueHint::AnyPath)]
        name: String,
    },

    /// Show a password
    #[command(alias = "s")]
    Show {
        /// Show only this key;
        /// "password" shows the first line;
        /// "otp" generates the one-time password
        #[arg(short, long, value_name = "otp|user|email|...")]
        key: Option<String>,

        /// Add the value to the clipboard
        #[arg(short, long)]
        clip: bool,

        /// Name of the password or directory
        #[arg(index = 1, default_value_t = String::from(""), value_hint = ValueHint::FilePath)]
        name: String,
    },

    /// Move a password
    Mv {
        /// Old name of the password or directory
        #[arg(index = 1, value_hint = ValueHint::AnyPath)]
        old_name: String,

        /// New name of the password or directory
        #[arg(index = 2, value_hint = ValueHint::AnyPath)]
        new_name: String,
    },

    /// Remove a password
    Rm {
        /// For directories
        #[arg(short, long)]
        recursive: bool,

        /// Name of the password or directory
        #[arg(index = 1, value_hint = ValueHint::AnyPath)]
        name: String,
    },

    /// Launch a menu to select a password and type/clip it
    #[command(name = "menu")]
    MenuCmd {
        /// The dmenu-like program used to select a password
        #[arg(long, value_name = "MENU PROGRAM")]
        menu_program: Option<String>,

        /// The program that is used to type
        #[arg(long, value_name = "TYPING PROGRAM")]
        typing_program: Option<String>,

        /// Is passed on to the typing program to set a custom delay between keystrokes
        #[arg(short = 'd', long, value_name = "MILLISECONDS")]
        key_delay: Option<u16>,

        /// Copy or type contents of a password; Can be chained
        //#[command(subcommand, action = clap::ArgAction::Append)]
        //action_args: SeniormenuArg,
        #[arg(
            value_name = "ACTIONS",
            trailing_var_arg = true,
            allow_hyphen_values = true,
            required = true,
            help = "[clip <KEY> | type-content <KEY> | type-text <TEXT> | sleep <MILLISECONDS>]..."
        )]
        action_args: Vec<String>,
    },

    /// Print the directory of the store
    PrintDir,

    /// Run git commands in the store
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

    /// Change the store's passphrase
    ChangePassphrase,

    /// Search the contents of each password file
    Grep {
        /// The regex pattern or the command that should be used for searching
        pattern_or_cmd: String,
        /// Arguments for the command that is used for searching
        #[arg(allow_hyphen_values = true, trailing_var_arg = true, value_hint = ValueHint::CommandWithArguments)]
        args: Vec<String>,
    },

    /// Show the contents of all password files
    Cat {
        /// Optionally restrict to a single directory
        #[arg(index = 1, default_value_t = String::from(""), value_hint = ValueHint::FilePath)]
        dirname: String,
    },

    /// Unlock a store without showing any password
    Unlock {
        /// Do not prompt to unlock; Return an error if the store is locked;
        /// Useful for scripts
        #[arg(long)]
        check: bool,
    },

    /// Start the agent to cache the passphrases for your identity files
    Agent {
        /// Passphrase is cleared from the agent after n seconds;
        /// The timer is reset each time the passphrase is accessed.
        #[arg(long, default_value_t = 600, value_name = "SECONDS")]
        default_cache_ttl: u64,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum SeniormenuArg {
    /// Add the value to the clipboard
    Clip {
        /// The key that should be clipped;
        /// "password" clips the first line;
        /// "otp" generates the one-time password
        #[arg(index = 1, value_name = "password|otp|user|email|...")]
        key: String,
    },
    /// Type the value
    TypeContent {
        /// The key that should be typed;
        /// "password" clips the first line;
        /// "otp" generates the one-time password
        #[arg(index = 1, value_name = "password|otp|user|email|...")]
        key: String,
    },
    /// Type text
    TypeText {
        /// The text that should be typed;
        #[arg(index = 1, value_name = "TEXT")]
        text: String,
    },
    /// Wait
    Sleep {
        /// The number of milliseconds to wait
        #[arg(index = 1, value_name = "MILLISECONDS")]
        delay: u16,
    },
}
