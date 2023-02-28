.DEFAULT_GOAL = install

.PHONY: build
build:
	cargo build --release

.PHONY: install
install: build
	cargo install --root ~/.local --path .
