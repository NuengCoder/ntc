PREFIX ?= /usr
DESTDIR ?=

all:
	cargo build --release

clean:
	cargo clean

install:
	install -Dm755 target/release/ntc $(DESTDIR)$(PREFIX)/bin/ntc

.PHONY: all clean install
