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

db: has_secrets running_postgres
		@echo "Starting postgres container"
		$$(mkdir -p $$HOME/postgres-data/)
		docker run --rm -d \
			--name dev-postgres \
			-p "5432:5432" \
			--expose 5432 \
			-e POSTGRES_DB="$$POSTGRES_DB" \
			-e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
			-e POSTGRES_USER="$$POSTGRES_USER" \
			-v $$HOME/postgres-data/:/var/lib/postgresql/data \
			postgres

setup:
		@echo $(VERSION)
		cargo install diesel_cli --no-default-features --features postgres
		rustup override set $$(cat .rustup)

run: has_secrets has_postgres
		cargo run
