projects = $(shell perl -0pe 's/.*members\s*=\s*\[\s*(.+?)\].*/\1/smg && s/[ ",]//g' ./Cargo.toml)

build:
	cargo build

define test-target
  test:: ; cd $1 && cargo test --all-features
endef

$(foreach dir,$(projects),$(eval $(call test-target,$(dir))))