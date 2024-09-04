# Onur is free software: you can redistribute it and/or modify
# it under  the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# Onur is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Onur. If not, see <https://www.gnu.org/licenses/>.

# DEPS: fzf podman bash:5.5

.DEFAULT_GOAL := test

RUNNER ?= podman
NAME := onur
VERSION := $(shell gawk -F '"' '/version/ { print $$2; exit}' Cargo.toml)
CONTAINER_IMAGE := registry.gitlab.com/${USER}/${NAME}-rust:${VERSION}

# ================================= TASKS

.PHONY: local.build
local.build:
	cargo build --release --verbose

.PHONY: local.test
local.test:
	cargo test --verbose

.PHONY: local.deps
local.deps:
	cargo build

.PHONY: local.fmt
local.fmt:
	cargo fmt

.PHONY: local.lint
local.lint:
	cargo clippy

.PHONY: local.pub
local.pub:
	cargo publish

.PHONY: local.install
local.install: build
	cp ./target/release/onur ${HOME}/.local/bin/

# ================================= CONTAINER

.PHONY: image.build
image.build:
	${RUNNER} build --file ./Containerfile --tag ${CONTAINER_IMAGE} --env ONUR_VERSION=${VERSION}

.PHONY: image.repl
image.repl:
	${RUNNER} run --rm -it \
		--volume ${PWD}:/app:Z \
		--workdir /home/easbarba/app \
		${CONTAINER_IMAGE} bash

.PHONY: image.publish
image.publish:
	${RUNNER} push ${CONTAINER_IMAGE}

.PHONY: image.commands
image.commands:
	${RUNNER} run --rm -it \
		--volume ${PWD}:/app:Z \
		--workdir /home/easbarba/app \
		${CONTAINER_IMAGE} bash -c "$(shell cat ./container-commands | fzf)"


