.PHONY: lib all test clean

LIBNAME := $(shell rustc --crate-file-name src/zbase32/lib.rs)

all: lib zbase32enc

lib: lib/$(LIBNAME)

lib/$(LIBNAME): src/zbase32/lib.rs
	@mkdir -p lib
	rustc -O --out-dir lib $<

zbase32enc: src/zbase32enc.rs
	@mkdir -p bin
	rustc -O --out-dir bin -L lib $<

test:
	rustc --test src/zbase32/lib.rs
	./zbase32