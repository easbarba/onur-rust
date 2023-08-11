.DEFAULT_GOAL := test

RUNNER ?= podman
NAME := qas
VERSION := $(shell gawk '/version/ {version=substr($$3,2,5); print version; exit}' Cargo.toml)
FULLNAME := ${USER}/${NAME}:${VERSION}

# -------------------------------- CONTAINERS

image-test:
	${RUNNER} run --rm ${FULLNAME}

image-build:
	${RUNNER} build --file ./Containerfile --tag ${FULLNAME}

image-clean:
	${RUNNER} container --rm -f ${FULLNAME}


# --------------------- TASKS

build:
	cargo build --release --verbose

test:
	cargo test --verbose

deps:
	cargo build

fmt:
	cargo fmt

lint:
	cargo clippy

pub:
	cargo publish

# -------------------------------- COMMANDS

grab:
	cargo run -- grab --verbose

backup:
	cargo run -- backup --name nuxt,awesomewm --verbose

.PHONY: deps test fmt lint pub run build grab archive image-build image-clean image-test
