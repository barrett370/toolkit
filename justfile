PREFIX := env('HOME') / '.local'

default: build

clean:
	cargo clean

build:
	cargo build --release

install: build
	cargo install --root {{PREFIX}} --path .
