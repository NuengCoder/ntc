PREFIX ?= /usr
DESTDIR ?=
MIN_GLIBC ?= 2.39
BIN := target/release/ntc

UNAME_S := $(shell uname -s 2>/dev/null || echo unknown)

all: check
	cargo build --release

check: check-deps check-glibc

check-deps:
	@missing=0; \
	for tool in cargo rustc; do \
		if ! command -v $$tool >/dev/null 2>&1; then \
			echo "error: $$tool not found. Install Rust: https://rustup.rs" >&2; \
			missing=1; \
		fi; \
	done; \
	if [ ! -f ntc_web/dist/index.html ] || [ ! -f ntc_chess_web/dist/index.html ]; then \
		echo "error: frontend dist missing. Build it first:" >&2; \
		echo "  cd ntc_web && npm ci && npm run build" >&2; \
		echo "  cd ntc_chess_web && npm ci && npm run build" >&2; \
		missing=1; \
	fi; \
	if [ "$(UNAME_S)" = "Linux" ]; then \
		if ! command -v pkg-config >/dev/null 2>&1; then \
			echo "error: pkg-config not found (needed by the ALSA audio backend)." >&2; \
			missing=1; \
		elif ! pkg-config --exists alsa; then \
			echo "error: ALSA headers not found." >&2; \
			echo "  Debian/Ubuntu: sudo apt-get install build-essential pkg-config libasound2-dev" >&2; \
			echo "  Fedora:        sudo dnf install gcc pkg-config alsa-lib-devel" >&2; \
			echo "  Arch:          sudo pacman -S --needed base-devel pkgconf alsa-lib" >&2; \
			missing=1; \
		elif ! pkg-config --exists bzip2; then \
			echo "error: bzip2 headers not found." >&2; \
			echo "  Debian/Ubuntu: sudo apt-get install libbz2-dev" >&2; \
			echo "  Fedora:        sudo dnf install bzip2-devel" >&2; \
			echo "  Arch:          sudo pacman -S --needed bzip2" >&2; \
			missing=1; \
		elif ! pkg-config --exists liblzma; then \
			echo "error: xz/lzma headers not found." >&2; \
			echo "  Debian/Ubuntu: sudo apt-get install liblzma-dev" >&2; \
			echo "  Fedora:        sudo dnf install xz-devel" >&2; \
			echo "  Arch:          sudo pacman -S --needed xz" >&2; \
			missing=1; \
		fi; \
	fi; \
	exit $$missing

check-glibc:
	@if [ "$(UNAME_S)" = "Linux" ]; then \
		glibc="$$(getconf GNU_LIBC_VERSION 2>/dev/null | awk '{print $$2}')"; \
		[ -n "$$glibc" ] || glibc="$$(ldd --version 2>/dev/null | awk 'NR==1 {print $$NF}')"; \
		if [ -z "$$glibc" ]; then \
			echo "  warning: glibc not detected (non-glibc system); skipping check." >&2; \
		elif [ "$$(printf '%s\n%s\n' '$(MIN_GLIBC)' "$$glibc" | sort -V | head -n1)" != "$(MIN_GLIBC)" ]; then \
			echo "error: glibc $$glibc is too old; ntc needs >= $(MIN_GLIBC)." >&2; \
			echo "  Build on a newer distro, or install a release binary built for your glibc." >&2; \
			exit 1; \
		else \
			echo "  glibc $$glibc OK (>= $(MIN_GLIBC))"; \
		fi; \
	fi

install: check-glibc
	@test -f $(BIN) || { echo "error: $(BIN) not found. Run 'make' first." >&2; exit 1; }
	install -Dm755 $(BIN) $(DESTDIR)$(PREFIX)/bin/ntc

clean:
	cargo clean

.PHONY: all check check-deps check-glibc clean install
