CARGO=cargo

.PHONY: test
test:
	$(CARGO) test --no-default-features --features http,macros
