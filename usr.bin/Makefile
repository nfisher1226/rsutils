SUBDIRS += base32
SUBDIRS += base64
SUBDIRS += basename
SUBDIRS += dirname
SUBDIRS += factor
SUBDIRS += fold
SUBDIRS += nproc
SUBDIRS += readlink
SUBDIRS += rev
SUBDIRS += wc
SUBDIRS += yes

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
