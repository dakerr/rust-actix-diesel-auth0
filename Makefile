NAME=backend-rs
VERSION=$(shell git rev-parse HEAD)
REPO=dakerr
SHELL := /bin/bash

has_secrets:
		@[[ $$POSTGRES_DB ]] || (echo "source env.sh first"; exit 2)

has_postgres:
		@[ -n "$$(docker ps -q -f name="dev-postgres")" ] || (echo "db not running"; exit 2)

running_postgres:
		@[ -z "$$(docker ps -q -f name="dev-postgres")" ] || (echo "db already running": exit 2)

running_network:
		@[ -n "$$(docker network ls -q -f name="backend-net")" ] || (echo "network not running": exit 2)

network:
		docker network create backend-net

db: has_secrets running_postgres running_network
		@echo "Starting postgres container"
		$$(mkdir -p $$HOME/postgres-data/)
		docker run --rm -d \
			--network backend-net --network-alias pgdb \
			--name dev-postgres \
			-p "5432:5432" \
			--expose 5432 \
			-e POSTGRES_DB="$$POSTGRES_DB" \
			-e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
			-e POSTGRES_USER="$$POSTGRES_USER" \
			-v $$HOME/postgres-data/:/var/lib/postgresql/data \
			postgres

compile:
		docker run --rm \
			--network backend-net \
			-v cargo-cache:/root/.cargo \
			-v $$PWD:/volume \
			-w /volume \
			-it rust:1.52 \
			cargo build --release
		strip target/release/backend-rs
		mv target/release/backend-rs .

setup:
		@echo $(VERSION)
		cargo install diesel_cli --no-default-features --features postgres
		rustup override set $$(cat .rustup)

run: has_secrets has_postgres
		cargo run
