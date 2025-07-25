![seniorpw](other/logo.svg)

A Password Manager Using [age](https://github.com/FiloSottile/age) for Encryption

![seniorpw demonstration](other/seniorpw-demo.svg)

## Contents
- [Features](#features)
- [Comparison with Alternatives](#comparison-with-alternatives)
- [Usage](#usage)
- [Install](#install)
- [Import From Another Password Manager](#import-from-another-password-manager)
- [How It Works](#how-it-works)

## Features
It is inspired by [pass](https://www.passwordstore.org/).
seniorpw's features are
- Multiple stores
- OTP support
- Clipboard support for [Linux](https://kernel.org/) ([Wayland](https://wayland.freedesktop.org/) and [X11](https://www.x.org/wiki/)), [Termux](https://termux.dev/en/), [WSL](https://learn.microsoft.com/en-us/windows/wsl/about), [Darwin](https://opensource.apple.com/) ([macOS](https://www.apple.com/macos/))
- Select and automatically copy or type ([ydotool](https://github.com/ReimuNotMoe/ydotool) or [xdotool](https://github.com/jordansissel/xdotool)) a password via `seniormenu`
- git support
- Completions for bash and zsh
- Passphrase protected identities
- Passphrases only need to be entered once per session and then get cached by `senior-agent`
- A store can be shared among a group (encryption for multiple recipients)
- Search (grep) inside the passwords
- No config files
- Symlinks between stores are supported

To do:
- Android app
- Browser Add-On
- More import scripts

### Comparison with Alternatives
| Name | Backend | Encrypted Identities | Agent | git | TOTP | Configless | Language |
| - | - | - | - | - | - | - | - |
| [pasejo](https://github.com/metio/pasejo) | age | ❌ | - | ✅ | ✅ | ❌ | Rust |
| [psswd](https://github.com/Gogopex/psswd) | age | ❌ scrypt | ❌ | ❌ | ❌ | ✅ | Rust |
| [Pa-rs E](https://gitlab.com/mchal_/parse) | - | - | - | - | ❌ | ❌ | Rust |
| [privage](https://github.com/revelaction/privage) | age | yubikey | - | ✅ | ❌ | ✅ | Go |
| [neopass](https://github.com/nwehr/neopass) | age | ✅ | ❌ | - | ❌ | ❌ | Go |
| [pa](https://passwordass.org/) | age | ✅ | ❌ | ✅ | ❌ | ✅ | POSIX Shell |
| [passage](https://github.com/FiloSottile/passage) | age | ✅ | ❌ | ✅ | [✅](https://github.com/tadfisher/pass-otp/pull/178) | ✅ | Bash |
| [kbs2](https://github.com/woodruffw/kbs2) | age | ✅ | ✅ | ❌ | ❌ | ❌ | Rust |
| [pago](https://github.com/dbohdan/pago) | age | ✅ | ✅ | ✅ | ❌ | ✅ | Go |
| [pass](https://www.passwordstore.org/) | gpg | ✅ | ✅ gpg-agent | ✅ | [✅](https://github.com/tadfisher/pass-otp) | ✅ | Bash |
| [seniorpw](https://gitlab.com/retirement-home/seniorpw) | age | ✅ | ✅ | ✅ | ✅ | ✅ | Rust |

## Usage
### Create a New Store
```sh
senior init
# optionally initialise for git use:
senior git init
senior git add '*'
senior git commit -m "init"
```
The default store name is `main`. You can use `senior -s <NAME> <command>` to use another name.

### git-clone an Existing Store
```sh
senior clone git@gitlab.com:exampleuser/mystore.git
```
Without specifying another store name (using `-s`), the default name will be `mystore` in this example.
Someone who already has access to the store can then add you to the recipients via
```sh
senior add-recipient <PUBLIC KEY> <ALIAS>
```

### Use an Existing Identity
Both `senior init` and `senior clone` support the optional flag `-i <FILE>` or `--identity <FILE>`
to use an existing identity instead of generating a new one.
Supported are
- Cleartext age identity
- Passphrase encrypted age identity
- ssh key of type ed25519 or rsa

### Edit/Show/Move/Remove a Password
```sh
senior edit example.com
senior show example.com
senior mv example.com example2.com
senior rm example2.com
```
`senior show` has the option `-k` or `--key` to only print the value of a `key: value` pair.
The special key `otp` creates the one-time password from the otpauth-string.
```sh
$ senior show example.com
mysecretpassword
user: myusername
otpauth://totp/Example:alice@google.com?secret=JBSWY3DPEHPK3PXP&issuer=Example
# use `-c` or `--clip` to also add it to the clipboard
$ senior show -c -k user example.com
myusername
$ senior show -k otp example.com
118250
```

### git Support
With `senior git` you can run git commands in the `senior print-dir` directory.
If you have initialised your store for git use then
any `senior edit` creates a git-commit.
To sync it with remote, run
```sh
senior git pull
senior git push
```

### Multiple Stores
You can use multiple stores by setting `-s` or `--store`
```sh
$ ls "$(senior print-dir)"/..
friends  main  work
# the default store is `main`
$ senior show
/home/bob/.local/share/senior/main
├── gitlab.com
├── friends -> ../friends
│   ├── amazon.com
│   ├── example.com
│   └── netflix.com
└── gitlab.com
$ senior -s friends show
/home/bob/.local/share/senior/friends
├── amazon.com
├── example.com
└── netflix.com
$ senior -s work show
/home/bob/.local/share/senior/work
├── server1
└── workstation
```
Notice the symlink `main/friends -> ../friends`. This makes the two commands
```sh
$ senior -s friends show example.com
$ senior show friends/example.com
```
equivalent.
seniorpw recognises that `main/friends/example.com` is actually at `friends/example.com` and therefore uses
`friends/.identity.age` to decrypt.
The same goes for `senior edit` and using `friends/.recipients/*` to encrypt.
This is very practical for [seniormenu](#seniormenu), as it only looks inside the default store.

If only one store exists then this is the default store. Otherwise, `main` is the default store.

### seniormenu
```
seniormenu [--menu <dmenu-wl>] [--dotool <ydotool>] [--type] [<key1> <key2> ...]
```
seniormenu uses `dmenu-wl` or `dmenu` (can be changed with `--menu <othermenu>`) to let you select a password for the clipboard.
You can provide a `<key>` to get another value from the password file (like user, email, ...).

With `--type` the password gets typed using [ydotool](https://github.com/ReimuNotMoe/ydotool) (for Wayland) / [xdotool](https://github.com/jordansissel/xdotool) (for X11). The default can be changed with `--dotool <otherdotool>`.

ydotool feature only: You can specify multiple keys. Inbetween keys, a TAB is typed. After typing the password or the otp, the ENTER key gets pressed.

Set up some keybindings in your window manager to quickly clip/type passwords.
An example for sway/i3 is
```
bindsym $mod+u exec seniormenu --menu bemenu --type
bindsym $mod+y exec seniormenu --menu bemenu --type otp
bindsym $mod+t exec seniormenu --menu bemenu --type user password
```

### senior-agent
If you have set a passphrase to protect your identity file, then running
`age -d -i .identity.age example.com.age`
would require you to enter the passphrase each time.
Because this is very cumbersome, seniorpw provides an agent.

Upon receiving your passphrase once,
`senior` starts `senior-agent` to cache your identity.
This way you only have to enter your passphrase once per session.

### Search the Password Contents
There are two possibilities: `senior grep` and `senior cat`.
```sh
senior grep <REGEX PATTERN>
```
This searches the password contents of an entire store for a regex pattern.
Alternatively, use a custom pattern matching program for more sophisticated
searches.
```sh
# using the system's grep
senior grep grep --color=always -i -n <REGEX PATTERN>
# using fzf
senior grep fzf --filter=<REGEX PATTERN> --no-sort
# using ripgrep
senior grep rg --color=always -i -n <REGEX PATTERN>
```

Use `senior cat` to print the contents of the entire store or a subdirectory.
Pipe the output to your favourite pattern matching program.
```sh
senior cat | less
senior cat [dirname] | fzf
senior cat [dirname] | grep -C 5 -i -n <REGEX PATTERN>
```

## Install
### Arch BASED Systems
Simply use the provided [PKGBUILD](PKGBUILD).
```sh
# Download the PKGBUILD into an empty directory
curl -O "https://gitlab.com/retirement-home/senior/-/raw/main/PKGBUILD"
# Install the package with all its dependencies
makepkg -sic
```

### Other Systems
```sh
# build
make

# install
sudo make install

# uninstall
sudo make uninstall
```
On Termux you should omit the `sudo`.
Make sure you have the dependencies installed (look at `depends` and `makedepends` in the [PKGBUILD](PKGBUILD)).

## Import From Another Password Manager
### [pass](https://git.zx2c4.com/password-store/)
Use the script [pass2seniorpw.py](src/importers/pass2seniorpw.py) to import your passwords.
```sh
./pass2seniorpw.py ~/.password-store "$(senior print-dir)"
# set a passphrase
senior change-passphrase
```

### [KeePass](https://keepassxc.org/)
First export your database as a CSV file, then use the script
[keepass2seniorpw.py](src/importers/keepass2seniorpw.py) to import your
passwords.
```sh
./keepass2seniorpw.py exported-passwords.csv "$(senior print-dir)"
# set a passphrase
senior change-passphrase
```

## How It Works
Your store is just a directory, usually `~/.local/share/senior/main/`. Run `senior print-dir` to find out.
Let us look at the directory tree.
```sh
$ tree -a "$(senior print-dir)"
/home/bob/.local/share/senior/main
├── example.com.age
├── .gitignore
├── gitlab.com.age
├── .identity.age
└── .recipients
    └── main.txt
```
Apart from `.gitignore` there are two special entries: `.identity.age` and `.recipients/`.

- `.identity.age` is your age identity that is used to decrypt the passwords.

- `.recipients/main.txt` contains the public keys used for encrypting the passwords.

The passwords are age-encrypted text files.
Let us look at a password:
```sh
$ senior show gitlab.com
mysupersafepassword
user: myusername
```
The `show` command is equivalent to
```sh
$ age -d -i .identity.age gitlab.com.age
mysupersafepassword
user: myusername
```

With `senior edit ...`, after editing the decrypted text file, it gets encrypted via
```sh
$ age -e -R .recipients/main.txt -o gitlab.com.age /tmp/gitlab.com.txt
```

