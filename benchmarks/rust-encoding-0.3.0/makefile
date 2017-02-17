.PHONY: all touch clean

all:
	$(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)
touch:
	touch src/lib.rs
clean:
	$(CARGO) clean
patches:
	@echo ''
