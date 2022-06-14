RELEASE_OR_DEBUG ?= debug

.PHONY: build

build:
	cargo build `[ "${RELEASE_OR_DEBUG}" = "release" ] && echo "--release"`
