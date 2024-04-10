.PHONY: build prove run verify

build:
	cargo build

prove:
	cargo nexus prove

run:
	cargo nexus run

verify:
	cargo nexus verify
