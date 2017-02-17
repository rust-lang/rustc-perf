.PHONY: all touch clean

all:
	$(CARGO) rustc $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)
touch:
	find . -name '*.rs' | xargs touch
clean:
	$(CARGO) clean
patches:
	@echo ''
