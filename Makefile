.PHONY: build prove run verify

build:
	cargo build

prove:
	cargo nexus prove --bin $(bin)

run:
	cargo nexus run --bin $(bin)

verify:
	cargo nexus verify --bin $(bin)
