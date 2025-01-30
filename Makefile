CARGO=cargo

.PHONY: test
test:
	$(CARGO) test --no-default-features --features http,macros

.PHONY: fmt
fmt:
	$(CARGO) fmt --all -- --check

.PHONY: clippy
clippy:
	$(CARGO) clippy --all-features -- -W clippy::pedantic -D warnings