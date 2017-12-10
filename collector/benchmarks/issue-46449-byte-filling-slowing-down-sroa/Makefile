.PHONY: all@010-prepare \
		patches

# Make the depedencies available. Should finish very quickly.
all@010-prepare:
	$(CARGO) rustc --release $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# Set length to 6144 using std::io::Error as error.
# Fast (~1s) on 1.22.1, Slow (~16s) on 1.23.0-beta.2.
all@020-io-error-6144:
	patch -Np4 -i 020-io-error-6144.diff
	$(CARGO) rustc --release $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# Change error type to u32, reduce length to 3072 to avoid taking too long time.
# Slow (~21s) on 1.22.1, Fast (~1s) on 1.23.0-beta.2.
all@030-u32-3072:
	patch -Np4 -i 030-u32-3072.diff
	$(CARGO) rustc --release $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# Change error type to u8.
# Slow (~21s) on 1.22.1, Fast (~1s) on 1.23.0-beta.2.
all@031-u8-3072:
	patch -Np4 -i 031-u8-3072.diff
	$(CARGO) rustc --release $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# Change error type to empty.
# Slow (~20s) on 1.22.1, Very slow (~27s) on 1.23.0-beta.2.
all@040-empty-3072:
	patch -Np4 -i 040-empty-3072.diff
	$(CARGO) rustc --release $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

# Change error type to &str, increase length back to 6144 as this one is faster.
# Slow (~22s) on 1.22.1, Medium (~8s) on 1.23.0-beta.2.
all@050-static-str-6144:
	patch -Np4 -i 050-static-str-6144.diff
	$(CARGO) rustc --release $(CARGO_OPTS) -- $(CARGO_RUSTC_OPTS)

patches:
	@echo "\
		@010-prepare \
		@020-io-error-6144 \
		@030-u32-3072 \
		@031-u8-3072 \
		@040-empty-3072 \
		@050-static-str-6144 \
	"
