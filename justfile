#!/usr/bin/env just --justfile

install_deps:
    cargo install sqlx-cli

run_aux_services:
	docker compose up -d

run: run_aux_services
    cargo run --color=always --package platform-rs --bin platform-rs

test: run_aux_services
    @echo "Let's give the DB a chance to start up..." && sleep 5
    sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/postgres
    cargo test --bin platform-rs

reset_db:
	docker compose down --volumes
