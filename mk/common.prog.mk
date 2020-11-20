BINDIR     ?= $(DESTDIR)$(PREFIX)/bin
DATADIR    ?= $(DESTDIR)$(PREFIX)/share
MANDIR     ?= $(DATADIR)/share/man
INSTALLDIRS = $(BINDIR) $(MANDIR)/man1
VPATH      += src
VPATH      += man
VPATH      += target/release

all: $(PROGNAME)

$(PROGNAME): main.rs
	cargo build --release

install: $(BINDIR)/$(PROGNAME) $(MANDIR)/man1/$(PROGNAME).1

install-strip: $(BINDIR)/$(PROGNAME) $(MANDIR)/man1/$(PROGNAME).1
	strip -s $<

$(BINDIR)/$(PROGNAME): $(PROGNAME) | $(BINDIR)
	install -m0755 $< $@

$(MANDIR)/man1/$(PROGNAME).1: $(PROGNAME).1 | $(MANDIR)/man1
	install -m0644 $< $@

$(INSTALLDIRS):
	install -d $@

clean:
	rm -rf target/

uninstall:
	rm -rf $(BINDIR)/$(PROGNAME) $(MANDIR)/man1/$(PROGNAME).1

.PHONY: all clean install install-strip uninstall
