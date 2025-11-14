.PHONY: help build install uninstall

PREFIX ?= /usr
BINARY := $(PREFIX)/local/bin/senior
ZSHCOMPLETION := $(PREFIX)/local/share/zsh/site-functions/_senior
BASHCOMPLETION := $(PREFIX)/local/share/bash-completion/completions/senior
MANDIR := $(PREFIX)/local/share/man/man1

RUSTDIR := src/seniorpw

build: $(RUSTDIR)/target/release/senior src/man/senior.1

$(RUSTDIR)/target/release/senior src/man/senior.1: $(RUSTDIR)/src/*
	cargo build --manifest-path $(RUSTDIR)/Cargo.toml --bins --locked --release --target-dir $(RUSTDIR)/target

help:
	$(info run `make && sudo make install` or `sudo make uninstall`)

install: build
	mkdir -p $(shell dirname $(BINARY))
	mkdir -p $(shell dirname $(ZSHCOMPLETION))
	mkdir -p $(shell dirname $(BASHCOMPLETION))
	mkdir -p $(MANDIR)
	killall senior || true # Ignore error
	cp $(RUSTDIR)/target/release/senior $(BINARY)
	cp src/completions/senior.zsh $(ZSHCOMPLETION)
	cp src/completions/senior.bash $(BASHCOMPLETION)
	cp src/man/* $(MANDIR)

uninstall:
	rm -f $(BINARY)
	rm -f $(PREFIX)/local/bin/senior-agent
	rm -f $(PREFIX)/local/bin/seniormenu
	rm -f $(ZSHCOMPLETION)
	rm -f $(BASHCOMPLETION)
	rm -f $(MANDIR)/senior*.1

