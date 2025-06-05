#!/usr/bin/env python3
import csv
import os
import socket
import subprocess
import sys

def find_age_backend():
    known_backends = ["rage", "age"]
    for backend in known_backends:
        s = subprocess.run(["which", backend], capture_output=True)
        if s.returncode == 0:
            return backend
    raise Exception("No valid age backend installed? Please install one of the following: {}".format(known_backends))

def main():
    csv_file = None
    target_dir = None

    if "-h" in sys.argv[1:] or "--help" in sys.argv[1:] or len(sys.argv[1:]) != 2:
        print("Usage: {} <csv-file> <target-dir>".format(sys.argv[0]))
        sys.exit()

    csv_file = sys.argv[1]
    target_dir = sys.argv[2]
    if os.path.exists(target_dir):
        raise Exception("{} exists already!".format(target_dir))

    age = find_age_backend()

    recipients_dir = os.path.join(target_dir, ".recipients")
    os.makedirs(recipients_dir, exist_ok=True)
    identity = os.path.join(target_dir, ".identity.txt")
    s = subprocess.run(["{}-keygen".format(age), "-o", identity], capture_output=True, text=True)
    s.check_returncode()
    public_key = s.stderr.strip()[len("Public key: "):]
    recipients_main = os.path.join(recipients_dir, "main.txt")
    with open(recipients_main, "w") as f:
        f.write("# {}@{}\n".format(os.environ["USER"], socket.gethostname()))
        f.write("{}\n".format(public_key))
    with open(os.path.join(target_dir, ".gitignore"), "w") as f:
        f.write("/.identity.*\n")

    with open(csv_file, newline="") as f:
        csv_reader = csv.reader(f)
        columns = next(csv_reader)
        c = dict()
        for i, col in enumerate(columns):
            c[col] = i
        for row in csv_reader:
            title = row[c["Title"]]
            if not title:
                continue
            group = row[c["Group"]]
            user = row[c["Username"]]
            password = row[c["Password"]]
            url = row[c["URL"]]
            notes = row[c["Notes"]]
            totp = row[c["TOTP"]]

            dirname = os.path.join(target_dir, group.split("/", 1)[1])
            filename = os.path.join(dirname, title + ".age")
            content = password
            if user:
                content += "\nuser: " + user
            if url:
                content += "\nurl: " + url
            if totp:
                content += "\n" + totp
            for col in columns:
                if col in ["Group", "Title", "Username", "Password", "URL", "Notes", "TOTP", "Icon", "Last Modified", "Created"]:
                    continue
                if row[c[col]]:
                    content += "\n" + col + ": " + row[c[col]]
            if notes:
                content += "\nnotes:"
                content += "\n" if "\n" in notes else " "
                content += notes
            content += "\n"

            os.makedirs(dirname, exist_ok=True)
            print(f"echo <content> | {age} -e -R {recipients_main} -o {filename}")
            age_encrypt = subprocess.Popen([age, "-e", "-R", recipients_main, "-o", filename], stdin=subprocess.PIPE, text=True)
            age_encrypt.communicate(content, timeout=15)
            assert age_encrypt.returncode == 0

    print("Done importing. Use `senior change-passphrase` to set a passphrase.")

if __name__ == "__main__":
    main()
