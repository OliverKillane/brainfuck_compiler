# Makefile used by labTS for using cargo.

RUSTFLAGS="-C link-args=-Wl,-zstack-size=104857600"

all: debug
debug:
	cargo build && mv target/debug/brainfuck_compiler bf
release:
	cargo build --release && mv target/release/brainfuck_compiler bf

docs:
	cargo doc && mv target/doc doc
launch_docs: docs
	 sensible-browser doc/brainfuck_compiler/index.html

# clean up all of the compiled files
clean:
	rm -rf docs && rm -f bf && cargo clean

.PHONY: all laucnh_docs clean
