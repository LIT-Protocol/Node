.PHONY: all
all: format clippy test

test:
	cd lit-core && make test
	cd lit-os && make test
	cd lit-node && make test

format:
	cd lit-core && make format
	cd lit-os && make format
	cd lit-node && make format

clippy:
	cd lit-core && make clippy
	cd lit-os && make clippy
	cd lit-node && make clippy
