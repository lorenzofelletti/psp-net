CARGO=cargo
EXAMPLES_DIR = examples

.PHONY: all
all: fmt clippy test build build-examples

.PHONY: test
test:
	$(CARGO) test --no-default-features --features http,macros

.PHONY: fmt
fmt:
	$(CARGO) fmt --all -- --check

.PHONY: clippy
clippy:
	$(CARGO) clippy --all-features -- -W clippy::pedantic -D warnings

.PHONY: build
build:
	$(CARGO) build


.PHONY: build-examples
build-examples: $(EXAMPLES_DIR)/*
	cd $^ && $(CARGO) psp --release
