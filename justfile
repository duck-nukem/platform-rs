#!/usr/bin/env just --justfile

install_deps:
    cargo install sqlx-cli

run_aux_services:
	docker compose up -d
	@sleep 1

run: run_aux_services
    cargo run --color=always --package platform-rs --bin platform-rs

make_migration *ARGS:
    sqlx migrate add {{ ARGS }}

migrate: run_aux_services
    sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/postgres

test: run_aux_services migrate
    cargo test --bin platform-rs

reset_db:
	docker compose down --volumes
