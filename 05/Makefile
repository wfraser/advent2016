RFLAGS=-O -C lto -L rust-crypto/target/release -L rust-crypto/target/release/deps

.PHONY:
all: prog

rust-crypto:
	[ -d rust-crypto ] \
		|| git clone --branch v0.2.36 https://github.com/DaGenix/rust-crypto \

prog: prog.rs rust-crypto rust-crypto/target/release/libcrypto.rlib
	rustc $(RFLAGS) prog.rs

rust-crypto/target/release/libcrypto.rlib: rust-crypto
	cd rust-crypto && cargo build --release

clean:
	rm -f prog
	rm -rf rust-crypto
