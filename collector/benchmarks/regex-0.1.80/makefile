.PHONY: all@010-baseline \
        all@020-incr-from-scratch \
        all@030-compile_one \
        all@040-is_valid_cap_letter \
        all@050-expand \
        all@060-Compiler-new \
        all@070-BYTE_FREQUENCIES \
        all@080-SparseSet \
        all@090-Job \
        all@100-incr-no-change \
        patches

all@010-baseline:
	$(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

all@020-incr-from-scratch:
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# `compile_one()` is a private method in a large CGU.
all@030-compile_one:
	patch -Np2 -i 030-compile_one.diff
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# `is_valid_cap_letter()` is a private function in a small CGU.
all@040-is_valid_cap_letter:
	patch -Np2 -i 040-is_valid_cap_letter.diff
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# `expand()` is a public function in a small CGU.
all@050-expand:
	patch -Np2 -i 050-expand.diff
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# `new()` is a public method in a large CGU called in one other module
all@060-Compiler-new:
	patch -Np2 -i 060-Compiler-new.diff
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# `BYTE_FREQUENCIES` is a public static used in one other module
all@070-BYTE_FREQUENCIES:
	patch -Np2 -i 070-BYTE_FREQUENCIES.diff
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# `sparse::SparseSet` is a struct used in two other modules
all@080-SparseSet:
	patch -Np2 -i 080-SparseSet.diff
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# `backtrack::Job` is a local enum used in one module
all@090-Job:
	patch -Np2 -i 090-Job.diff
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# re-compile with no actual change
all@100-incr-no-change:
	find . -name "*.rs" | xargs touch
	CARGO_INCREMENTAL=1 $(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

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
