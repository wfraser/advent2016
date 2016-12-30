RFLAGS=-O -C lto -L ../05/rust-crypto/target/release -L ../05/rust-crypto/target/release/deps

.PHONY:
all: prog

../05/rust-crypto:
	pushd ../05 \
		&& [ -d rust-crypto ] \
		|| git clone --branch v0.2.36 https://github.com/DaGenix/rust-crypto

prog: prog.rs ../05/rust-crypto ../05/rust-crypto/target/release/libcrypto.rlib
	rustc $(RFLAGS) prog.rs

../05/rust-crypto/target/release/libcrypto.rlib: ../05/rust-crypto
	pushd ../05/rust-crypto && cargo build --release

clean:
	rm -f prog
	rm -rf ../05/rust-crypto
