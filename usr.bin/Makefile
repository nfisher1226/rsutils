SUBDIRS += basename
SUBDIRS += factor

all: $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@

install:
	for dir in $(SUBDIRS) ; do $(MAKE) -C $${dir} install ; done

install-strip:
	for dir in $(SUBDIRS) ; do $(MAKE) -C $${dir} install-strip ; done


clean:
	for dir in $(SUBDIRS) ; do $(MAKE) -C $${dir} clean ; done

.PHONY: all $(SUBDIRS) clean install install-strip
