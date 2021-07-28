NAME=backend-rs
VERSION=$(shell git rev-parse HEAD)
REPO=dakerr
SHELL := /bin/bash

DOCKER_NET=backend-net
WEBAPP=rust-actix-diesel
DB_ALIAS=pgdb

include .env
export $(shell sed 's/=.*//' .env)

has_secrets:
		@[[ $$DB_NAME ]] || (echo "check .env"; exit 2)

has_postgres:
		@[ -n "$$(docker ps -q -a -f name="pgdb")" ] || (echo "db not running"; exit 2)

running_postgres:
		@[ -z "$$(docker ps -q -a -f name="pgdb")" ] || (echo "db already running": exit 2)

running_network:
		@[ -n "$$(docker network ls -q -f name="backend-net")" ] || (echo "network not running": exit 2)

network:
		docker network create backend-net

setup:
		@echo $(VERSION)
		cargo install diesel_cli --no-default-features --features postgres
		rustup override set $$(cat .rustup)

diesel_setup:
		diesel setup --database-url="$$DB_TYPE://$$DB_USER:$$DB_PASSWORD@$$DB_HOST/$$DB_NAME"

db: has_secrets running_postgres running_network
		@echo "Starting postgres container"
		$$(mkdir -p $$HOME/postgres-data/)
		docker run --rm -d \
			--network ${DOCKER_NET} --network-alias ${DB_ALIAS} \
			--name ${DB_ALIAS} \
			-p "5432:5432" \
			--expose 5432 \
			-e POSTGRES_DB="$$DB_NAME" \
			-e POSTGRES_PASSWORD="$$DB_PASSWORD" \
			-e POSTGRES_USER="$$DB_USER" \
			-v $$HOME/postgres-data/:/var/lib/postgresql/data \
			postgres

build:
		docker build -t $$WEBAPP .

# make sure DB_HOST is DB_ALIAS:5432å
run_container: has_secrets running_postgres running_network
		docker run --rm  -d \
			--network ${DOCKER_NET} \
			--name ${WEBAPP} \
			--env-file .env \
			-p "8080:8080" \
			${WEBAPP}

# make sure DB_HOST is localhost:5432
run_cargo: has_secrets has_postgres
		cargo run
