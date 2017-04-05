.PHONY: all touch clean \
	    all@010-baseline \
	    all@020-incr-from-scratch \
	    all@030-incr-change-private-fn

INCREMENTAL_FLAGS=-Z incremental=incr

all@010-baseline:
	$(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

all@020-incr-from-scratch:
	find . -name "*.rs" | xargs touch
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# `compile_one()` is a private method in a large CGU.
all@030-compile_one:
	patch -Np2 -i 030-compile_one.diff
	find . -name "*.rs" | xargs touch
	git diff
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# `is_valid_cap_letter()` is a private function in a small CGU.
all@040-is_valid_cap_letter:
	patch -Np2 -i 040-is_valid_cap_letter.diff
	find . -name "*.rs" | xargs touch
	git diff
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# `expand()` is a public function in a small CGU.
all@050-expand:
	patch -Np2 -i 050-expand.diff
	find . -name "*.rs" | xargs touch
	git diff
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# `new()` is a public method in a large CGU called in one other module
all@060-Compiler-new:
	patch -Np2 -i 060-Compiler-new.diff
	find . -name "*.rs" | xargs touch
	git diff
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# `BYTE_FREQUENCIES` is a public static used in one other module
all@070-BYTE_FREQUENCIES:
	patch -Np2 -i 070-BYTE_FREQUENCIES.diff
	find . -name "*.rs" | xargs touch
	git diff
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# `sparse::SparseSet` is a struct used in two other modules
all@080-SparseSet:
	patch -Np2 -i 080-SparseSet.diff
	find . -name "*.rs" | xargs touch
	git diff
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# `backtrack::Job` is a local enum used in one module
all@090-Job:
	patch -Np2 -i 090-Job.diff
	find . -name "*.rs" | xargs touch
	git diff
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

# re-compile with no actual change
all@100-incr-no-change:
	find . -name "*.rs" | xargs touch
	RUSTFLAGS="${INCREMENTAL_FLAGS}" $(CARGO) rustc $(CARGO_OPTS) -- \
	  $(CARGO_RUSTC_OPTS) -Z incremental-info

patches:
	@echo "\
		@010-baseline \
	    @020-incr-from-scratch \
	    @030-compile_one \
	    @040-is_valid_cap_letter \
	    @050-expand \
	    @060-Compiler-new \
	    @070-BYTE_FREQUENCIES \
	    @080-SparseSet \
	    @090-Job \
	    @100-incr-no-change \
	"

touch:
	git checkout src
	find . -name "*.rs" | xargs touch
	rm -rf incr

clean:
	$(CARGO) clean
	rm -rf incr
