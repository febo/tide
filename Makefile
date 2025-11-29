RUST_TOOLCHAIN_NIGHTLY = nightly-2025-02-16
SOLANA_CLI_VERSION = 3.0.0

nightly = +${RUST_TOOLCHAIN_NIGHTLY}

# This is a bit tricky -- findstring returns the found string, so we're looking
# for "directory-", returning that, and replacing "-" with "/" to change the
# first "-" to a "/". But if it isn't found, we replace "" with "", which works
# in the case where there is no subdirectory.
pattern-dir = $(firstword $(subst -, ,$1))
find-pattern-dir = $(findstring $(call pattern-dir,$1)-,$1)
make-path = $(subst $(call find-pattern-dir,$1),$(subst -,/,$(call find-pattern-dir,$1)),$1)
# Convert 'programs/anything' to 'programs-anything'.
program-target = $(subst /,-,$(patsubst programs/%,programs-%,$1))
# All files directly inside programs.
PROGRAMS := $(wildcard programs/*)
# Generate the dashed target program names.
PROGRAM_TARGETS := $(foreach src,$(PROGRAMS),$(call program-target,$(src)))

rust-toolchain-nightly:
	@echo ${RUST_TOOLCHAIN_NIGHTLY}

solana-cli-version:
	@echo ${SOLANA_CLI_VERSION}

cargo-nightly:
	cargo $(nightly) $(ARGS)

clippy-%:
	cargo $(nightly) clippy --manifest-path $(call make-path,$*)/Cargo.toml \
	  --all-targets \
	  --all-features \
		-- \
		--deny=warnings \
		--deny=clippy::default_trait_access \
		--deny=clippy::arithmetic_side_effects \
		--deny=clippy::manual_let_else \
		--deny=clippy::used_underscore_binding $(ARGS)

format-check-%:
	cargo $(nightly) fmt --check --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS)

bench:
	cargo $(nightly) bench --manifest-path benchmark/Cargo.toml $(ARGS)

format-rust:
	cargo $(nightly) fmt --all $(ARGS)

build-sbf-%:
	cargo build-sbf --manifest-path $(call make-path,$*)/Cargo.toml $(ARGS) --tools-version v1.52

tests:
	SBF_OUT_DIR=$(PWD)/target/deploy cargo $(nightly) test --manifest-path benchmark/Cargo.toml $(ARGS)

all:
	@for dir in $(PROGRAM_TARGETS); do \
		$(MAKE) build-sbf-$$dir; \
	done
