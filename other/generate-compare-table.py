#!/usr/bin/env python3

class PasswordManager:
    def __init__(self, name: str, url: str, backend: str = "?", language: str = "?", agent: str = "?", agent_timeout: str = "?", encrypted_identities: str = "?", git: str = "?", key_value_pairs: str = "?", configless: str = "?", totp: str = "?"):
        self.name = name
        self.url = url
        self.backend = backend
        self.language = language
        self.agent = agent
        self.agent_timeout = agent_timeout
        self.encrypted_identities = encrypted_identities
        self.git = git
        self.key_value_pairs = key_value_pairs
        self.configless = configless
        self.totp = totp

    def __repr__(self):
        return f"| [{self.name}]({self.url}) | {self.backend} | {self.encrypted_identities} | {self.agent} | {self.agent_timeout} | {self.git} | {self.key_value_pairs} | {self.totp} | {self.configless} | {self.language} |"

def sort_pm(pm: PasswordManager):
    return ("seniorpw" in pm.name,
            not pm.encrypted_identities.startswith("❌"), pm.encrypted_identities,
            not pm.agent.startswith("❌"),
            not pm.agent_timeout.startswith("❌"),
            not pm.git.startswith("❌"), pm.git,
            not pm.key_value_pairs.startswith("❌"),
            not pm.totp.startswith("❌"), pm.totp,
            not pm.configless.startswith("❌"), pm.configless,
            pm.language, pm.backend, pm.name)

def pm_to_table(password_managers: list[PasswordManager]) -> str:
    ret = "| Name | Backend | Encrypted Identities | Agent | Agent Timeout | git | Key-Value Pairs | TOTP | Configless | Language |"
    ret += "\n| - | - | - | - | - | - | - | - | - | - |"
    for password_manager in sorted(password_managers, key = sort_pm):
        ret += f"\n{password_manager}"
    return ret

def main():
    password_managers: list[PasswordManager] = []
    password_managers.append(PasswordManager("Pa-rs E", "https://gitlab.com/mchal_/parse", backend="-", agent="-", encrypted_identities="-", git="-", language="Rust", configless="❌", totp="❌", agent_timeout="-", key_value_pairs="❌"))
    password_managers.append(PasswordManager("kbs2", "https://github.com/woodruffw/kbs2", backend="age", agent="✅", encrypted_identities="✅", git="❌", language="Rust", configless="❌", totp="❌", agent_timeout="❌", key_value_pairs="❌"))
    password_managers.append(PasswordManager("neopass", "https://github.com/nwehr/neopass", backend="age", agent="❌", encrypted_identities="✅", git="-", language="Go", configless="❌", totp="❌", agent_timeout="-", key_value_pairs="❌"))
    password_managers.append(PasswordManager("pa", "https://passwordass.org/", backend="age", agent="❌", encrypted_identities="✅", git="✅", language="POSIX Shell", configless="✅", totp="❌", agent_timeout="-", key_value_pairs="❌"))
    password_managers.append(PasswordManager("pago", "https://github.com/dbohdan/pago", backend="age", agent="✅", encrypted_identities="✅", git="✅", language="Go", configless="✅", totp="❌", agent_timeout="✅", key_value_pairs="✅"))
    password_managers.append(PasswordManager("pasejo", "https://github.com/metio/pasejo", backend="age", agent="-", encrypted_identities="❌", git="✅", language="Rust", configless="❌", totp="✅", agent_timeout="-", key_value_pairs="❌"))
    password_managers.append(PasswordManager("pass", "https://www.passwordstore.org/", backend="gpg", agent="✅ gpg-agent", encrypted_identities="✅", git="✅", language="Bash", configless="✅", totp="[✅](https://github.com/tadfisher/pass-otp)", agent_timeout="✅ gpg-agent", key_value_pairs="❌"))
    password_managers.append(PasswordManager("passage", "https://github.com/FiloSottile/passage", backend="age", agent="❌", encrypted_identities="✅", git="✅", language="Bash", configless="✅", totp="[✅](https://github.com/tadfisher/pass-otp/pull/178)", agent_timeout="-", key_value_pairs="❌"))
    password_managers.append(PasswordManager("privage", "https://github.com/revelaction/privage", backend="age", agent="-", encrypted_identities="yubikey", git="✅", language="Go", configless="✅", totp="❌", agent_timeout="-", key_value_pairs="❌"))
    password_managers.append(PasswordManager("psswd", "https://github.com/Gogopex/psswd", backend="age", agent="❌", encrypted_identities="❌ scrypt", git="❌", language="Rust", configless="✅", totp="❌", agent_timeout="-", key_value_pairs="❌"))
    password_managers.append(PasswordManager("seniorpw", "https://gitlab.com/retirement-home/seniorpw", backend="age", agent="✅", encrypted_identities="✅", git="✅", language="Rust", configless="✅", totp="✅", agent_timeout="❌", key_value_pairs="✅"))
    print(pm_to_table(password_managers))

if __name__ == "__main__":
    main()
