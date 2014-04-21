.PHONY: lib all test clean

LIBNAME := $(shell rustc --crate-file-name src/zbase32/lib.rs)

all: lib

lib: lib/$(LIBNAME)

lib/$(LIBNAME): src/zbase32/lib.rs
	@mkdir -p lib
	rustc -O $<

test:
	rustc --test src/zbase32/lib.rs
	./zbase32