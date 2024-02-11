projects = $(shell perl -0pe 's/.*members\s*=\s*\[\s*(.+?)\].*/\1/smg && s/[ ",]//g' ./Cargo.toml)

clippy:
	cargo clippy --all-features

format:
	cargo fmt

build:
	cargo build

define test-target
  test-all:: ; cd $1 && cargo test --all-features
endef

$(foreach dir,$(projects),$(eval $(call test-target,$(dir))))

# Specific testing due to feature flags.
test: test-all
	cd lit-cli && cargo test --no-default-features --features os-host-all,os-guest-all      # Prov
	cd lit-cli && cargo test --no-default-features --features os-host-all,os-guest-instance # Node
	cd lit-os-guest-initrd && cargo test --no-default-features --features type-prov,common  # Prov
	cd lit-os-guest-initrd && cargo test --no-default-features --features type-node,common  # Node
	cd lit-os-guest-initrd && cargo test --no-default-features --features type-custom,common  # Custom
