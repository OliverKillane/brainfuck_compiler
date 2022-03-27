# Makefile used by labTS for using cargo.

all: debug
debug:
	cargo build && mv target/debug/brainfuck_compiler bfc
release:
	cargo build --release && mv target/release/brainfuck_compiler bfc

docs:
	cargo doc
launch_docs: docs
	 sensible-browser target/doc/brainfuck_compiler/index.html

# clean up all of the compiled files
clean:
	rm -rf docs && rm -f bfc && cargo clean

.PHONY: all launch_docs clean
