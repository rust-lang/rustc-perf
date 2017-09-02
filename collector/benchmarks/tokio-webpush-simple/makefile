.PHONY: all

all@000-base:
	$(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

all@010-incr:
	find . -name '*.rs' | xargs touch
	CARGO_INCREMENTAL=1 \
	    $(CARGO) rustc $(CARGO_OPTS) -- \
	    $(CARGO_RUSTC_OPTS) -Z incremental-info

all@020-clean:
	find . -name '*.rs' | xargs touch
	CARGO_INCREMENTAL=1 \
	    $(CARGO) rustc $(CARGO_OPTS) -- \
	    $(CARGO_RUSTC_OPTS) -Z incremental-info

all@030-minor-change:
	patch -Np3 -i 030-minor-change.diff
	find . -name '*.rs' | xargs touch
	CARGO_INCREMENTAL=1 \
	    $(CARGO) rustc $(CARGO_OPTS) -- \
	    $(CARGO_RUSTC_OPTS) -Z incremental-info

patches:
	@echo '@000-base @010-incr @020-clean @030-minor-change'
