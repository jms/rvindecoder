#!make
SHELL = bash

build:
	cargo build --release

run:
	cargo run

clean:
	rm -fr target
	rm -fr release

load_data:
	./load-data.sh
