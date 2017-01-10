RFLAGS=-O -C lto -L linked-list/target/release -L linked-list/target/release/deps

.PHONY:
all: prog

linked-list:
	[ -d linked-list ] \
		|| git clone https://github.com/contain-rs/linked-list \

prog: prog.rs linked-list linked-list/target/release/liblinked_list.rlib
	rustc $(RFLAGS) prog.rs

linked-list/target/release/liblinked_list.rlib: linked-list
	cd linked-list && cargo build --release

clean:
	rm -f prog
	rm -rf linked-list
