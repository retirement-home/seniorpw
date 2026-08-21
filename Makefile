.PHONY: help build install uninstall

PREFIX ?= /usr
BINARY := $(PREFIX)/local/bin/senior
ZSHCOMPLETION := $(PREFIX)/local/share/zsh/site-functions/_senior
BASHCOMPLETION := $(PREFIX)/local/share/bash-completion/completions/senior
MANDIR := $(PREFIX)/local/share/man/man1

build: target/release/senior man/senior.1

target/release/senior man/senior.1: cli/src/*
	cargo build --manifest-path Cargo.toml --bins --locked --release --target-dir target

help:
	$(info run `make && sudo make install` or `sudo make uninstall`)

install: build
	mkdir -p $(shell dirname $(BINARY))
	mkdir -p $(shell dirname $(ZSHCOMPLETION))
	mkdir -p $(shell dirname $(BASHCOMPLETION))
	mkdir -p $(MANDIR)
	killall senior || true # Ignore error
	cp target/release/senior $(BINARY)
	cp completions/senior.zsh $(ZSHCOMPLETION)
	cp completions/senior.bash $(BASHCOMPLETION)
	cp man/* $(MANDIR)

uninstall:
	rm -f $(BINARY)
	rm -f $(PREFIX)/local/bin/senior-agent
	rm -f $(PREFIX)/local/bin/seniormenu
	rm -f $(ZSHCOMPLETION)
	rm -f $(BASHCOMPLETION)
	rm -f $(MANDIR)/senior*.1

