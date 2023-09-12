#!/usr/bin/env just --justfile

install_deps:
    cargo install sqlx-cli rust-i18n

run_aux_services:
	docker compose up -d --wait

run: run_aux_services migrate
    cargo run --color=always --package platform-rs --bin platform-rs

build_prod:
	cargo build --bin platform-rs --release

run_prod: build_prod run_aux_services migrate
	./target/release/platform-rs


make_migration *ARGS:
    sqlx migrate add {{ ARGS }}

migrate: run_aux_services
    sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/postgres

test: migrate
    cargo test --bin platform-rs

reset_db:
	docker compose down --volumes

get_auth_cookie:
    @curl -si 'http://localhost:3000/login' -X POST -H 'Content-Type: application/x-www-form-urlencoded' --data-raw 'username=admin&password=pass' | grep -o "sid=[^;]*"
