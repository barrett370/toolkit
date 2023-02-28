.DEFAULT_GOAL = install

.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build --release

.PHONY: install
install: build
	cargo install --root ~/.local --path .
