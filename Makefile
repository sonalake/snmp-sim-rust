BASE_IMAGE              ?= gcr.io/distroless/cc
BUILDER_IMAGE           ?= lukemathwalker/cargo-chef:latest-rust-1.60-slim
IMAGE                   ?= $(shell basename $(shell git rev-parse --show-toplevel))
IMAGE_TAG               ?= $(shell git describe --tags 2> /dev/null || echo latest)
RELEASE_OR_DEBUG        ?= release

.PHONY: all build run stop db-entity db-migrate rust-client

all: build

build:
	docker build -t snmp-sim \
	--build-arg "BASE_IMAGE=$(BASE_IMAGE)" \
	--build-arg "BUILDER_IMAGE=$(BUILDER_IMAGE)" \
	--build-arg "RELEASE_OR_DEBUG=$(RELEASE_OR_DEBUG)" \
	.

run:
	docker run -d --rm --name snmp-sim -p "127.0.0.1:8161:8161/udp" -p "127.0.0.1:8080:8080" -t snmp-sim

stop:
	docker stop snmp-sim

db-entity:
	sea-orm-cli generate entity -o ./src/data_access/entity
	sed -i 's/DateTime/DateTimeUtc/g' ./src/data_access/entity/*.rs

db-migrate:
	sqlx migrate run

rust-client:
	cd ./clients/rust
	cargo build
	cd ../..
