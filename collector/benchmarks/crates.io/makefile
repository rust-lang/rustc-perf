.PHONY: all

# at commit ef1ac581f9c4ea6a906347d43776418acfdbb8ca
# We must only benchmark the lib target because otherwise we can't pass args with CARGO_RUSTC_OPTS

all:
	$(CARGO) rustc $(CARGO_OPTS) --lib -- $(CARGO_RUSTC_OPTS)
patches:
	@echo ''
