.PHONY: all touch clean

all:
	$(CARGO) rustc --test all $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)
touch:
	rm -f target/debug/all-*
clean:
	$(CARGO) clean
patches:
	@echo ''
