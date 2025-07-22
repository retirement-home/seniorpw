#!/usr/bin/env python3
import argparse
import os
import os.path
import socket
import subprocess
import sys


def find_age_backend():
    known_backends = ["rage", "age"]
    for backend in known_backends:
        s = subprocess.run(["which", backend], capture_output=True)
        if s.returncode == 0:
            return backend
    raise Exception(
        "No valid age backend installed? Please install one of the following: {}".format(
            known_backends
        )
    )


def main():
    src_dir = None
    target_dir = None

    ap = argparse.ArgumentParser(
        prog=sys.argv[0], formatter_class=argparse.ArgumentDefaultsHelpFormatter
    )
    ap.add_argument(
        "--no-create",
        dest="create",
        default=True,
        action="store_false",
        help="Skip creation of target dir",
    )
    ap.add_argument("source", help="Password Store source dir")
    ap.add_argument("target", help="SeniorPW target dir")
    args = ap.parse_args()

    src_dir = os.path.expanduser(args.source)
    target_dir = os.path.expanduser(args.target)
    recipients_dir = os.path.join(target_dir, ".recipients")
    recipients_main = os.path.join(recipients_dir, "main.txt")

    age = find_age_backend()

    if args.create:
        if os.path.exists(target_dir):
            raise Exception("{} exists already!".format(target_dir))

        os.makedirs(recipients_dir, exist_ok=True)
        identity = os.path.join(target_dir, ".identity.txt")
        s = subprocess.run(
            ["{}-keygen".format(age), "-o", identity], capture_output=True, text=True
        )
        s.check_returncode()
        public_key = s.stderr.strip()[len("Public key: ") :]
        with open(recipients_main, "w") as f:
            f.write("# {}@{}\n".format(os.environ["USER"], socket.gethostname()))
            f.write("{}\n".format(public_key))
        with open(os.path.join(target_dir, ".gitignore"), "w") as f:
            f.write("/.identity.*\n")

    def copy_and_encrypt(dir_path):
        for filename in os.listdir(dir_path):
            filepath = os.path.join(dir_path, filename)
            if os.path.isdir(filepath):
                if filename == ".git":
                    print("Skipping {}".format(filepath), file=sys.stderr)
                    continue
                copy_and_encrypt(filepath)
                continue
            if not str(filename).endswith(".gpg"):
                print("Skipping {}".format(filepath), file=sys.stderr)
                continue
            target_parent = os.path.join(target_dir, dir_path[len(src_dir) + 1 :])
            os.makedirs(target_parent, exist_ok=True)
            target_path = os.path.join(target_parent, filename[:-4] + ".age")
            print(
                "gpg --decrypt {} | {} -e -R {} -o {}".format(
                    filepath, age, recipients_main, target_path
                )
            )
            gpg_decrypt = subprocess.Popen(
                ["gpg", "--decrypt", filepath],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            age_encrypt = subprocess.run(
                [age, "-e", "-R", recipients_main, "-o", target_path],
                stdin=gpg_decrypt.stdout,
            )
            gpg_decrypt.wait()
            if gpg_decrypt.returncode not in [0, 1]:
                raise Exception(
                    "Bad return code for gpg:\n{}".format(
                        gpg_decrypt.stderr.read()
                        if gpg_decrypt.stderr is not None
                        else ""
                    )
                )
            if age_encrypt.returncode != 0:
                raise Exception("Non-zero return code for age!")

    copy_and_encrypt(src_dir)
    print("Done importing.")
    if args.create:
        print("Use `senior change-passphrase` to set a passphrase.")


if __name__ == "__main__":
    main()
