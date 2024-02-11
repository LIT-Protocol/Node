# Use one of these commands to build the manifest for Node.js:
#
# - make
# - make DEBUG=1
# - make SGX=1
# - make SGX=1 DEBUG=1
#
# Use `make clean` to remove Graphene-generated files.

THIS_DIR := $(dir $(lastword $(MAKEFILE_LIST)))
NODEJS_DIR ?= /usr/bin
HOME_DIR = ${HOME}

# Relative path to Graphene root and key for enclave signing
GRAPHENEDIR ?= $(HOME_DIR)/graphene
SGX_SIGNER_KEY ?= $(GRAPHENEDIR)/Pal/src/host/Linux-SGX/signer/enclave-key.pem

ifeq ($(DEBUG),1)
GRAPHENE_LOG_LEVEL = debug
else
GRAPHENE_LOG_LEVEL = error
endif

.PHONY: all
all: lit_node.manifest
ifeq ($(SGX),1)
all: lit_node.manifest.sgx lit_node.sig lit_node.token
endif

include $(HOME_DIR)/graphene/Scripts/Makefile.configs

lit_node.manifest: lit_node.manifest.template
	graphene-manifest \
		-Dlog_level=$(GRAPHENE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		-Dnodejs_dir=$(NODEJS_DIR) \
		$< >$@

# Generate SGX-specific manifest, enclave signature, and token for enclave initialization
lit_node.manifest.sgx: lit_node.manifest target/debug/lit_node
	graphene-sgx-sign \
		--key $(SGX_SIGNER_KEY) \
		--manifest $< \
		--output $@

lit_node.sig: lit_node.manifest.sgx

lit_node.token: lit_node.sig
	graphene-sgx-get-token --output $@ --sig $^

ifeq ($(SGX),)
GRAPHENE = graphene-direct
else
GRAPHENE = graphene-sgx
endif


.PHONY: clean
clean:
	$(RM) *.manifest *.manifest.sgx *.token *.sig OUTPUT

.PHONY: distclean
distclean: clean