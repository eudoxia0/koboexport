PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin

.PHONY: all
all: koboexport

koboexport: Cargo.toml Cargo.lock src/main.rs
	cargo build --release
	cp target/release/koboexport koboexport

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
	cargo clean
