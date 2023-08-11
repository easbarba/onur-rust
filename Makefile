#  Copyright 2023 EAS Barbosa
#
#      Licensed under the Apache License, Version 2.0(the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.

.DEFAULT_GOAL := test

RUNNER ?= podman
NAME := onur
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
