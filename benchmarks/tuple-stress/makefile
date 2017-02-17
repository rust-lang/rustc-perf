.PHONY: all touch clean

all:
	$(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)
touch:
	touch src/main.rs
clean:
	$(CARGO) clean
