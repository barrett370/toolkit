.DEFAULT_GOAL = build

PREFIX ?= $(HOME)/.local

.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build --release

.PHONY: install
install: build
	cargo install --root $(PREFIX) --path .
