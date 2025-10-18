PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

.PHONY: all
all: koboexport

koboexport: Cargo.toml Cargo.lock src/main.rs
	cargo build --release --target-dir __build
	cp __build/release/koboexport koboexport
	rm -rf __build

.PHONY: install
install: koboexport
	install -d $(BINDIR)
	install -m 755 koboexport $(BINDIR)/koboexport

.PHONY: uninstall
uninstall:
	rm -f $(BINDIR)/koboexport

.PHONY: clean
clean:
	rm -f koboexport
	rm -rf __build
	cargo clean
