SUBDIRS = lit-core lit-os lit-node lit-actions

all: format clippy test

format: $(SUBDIRS)
	@for dir in $^; do $(MAKE) -C $$dir format; done

clippy: $(SUBDIRS)
	@for dir in $^; do $(MAKE) -C $$dir clippy; done

test: $(SUBDIRS)
	@for dir in $^; do $(MAKE) -C $$dir test; done

.PHONY: all format clippy test
