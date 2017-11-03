.PHONY: all

CARGO_OPTS += --features gecko --manifest-path components/style/Cargo.toml
CARGO_RUSTC_OPTS += --cap-lints=warn

all@000-base:
	$(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

all@010-incr:
	touch components/style/lib.rs
	CARGO_INCREMENTAL=1 \
	    $(CARGO) rustc $(CARGO_OPTS) -- \
	    $(CARGO_RUSTC_OPTS) -Z incremental-info

all@020-clean:
	touch components/style/lib.rs
	CARGO_INCREMENTAL=1 \
	    $(CARGO) rustc $(CARGO_OPTS) -- \
	    $(CARGO_RUSTC_OPTS) -Z incremental-info

all@030-commit1:
	patch -Np1 -i 030-commit-b5762295.patch
	touch components/style/lib.rs
	CARGO_INCREMENTAL=1 \
	    $(CARGO) rustc $(CARGO_OPTS) -- \
	    $(CARGO_RUSTC_OPTS) -Z incremental-info

all@040-commit2:
	patch -Np1 -i 040-commit-4e0492c5.patch
	touch components/style/lib.rs
	CARGO_INCREMENTAL=1 \
	    $(CARGO) rustc $(CARGO_OPTS) -- \
	    $(CARGO_RUSTC_OPTS) -Z incremental-info

patches:
	@echo '@000-base'
# TODO: reenable
#@echo '@000-base @010-incr @020-clean @030-commit1 @040-commit2'
