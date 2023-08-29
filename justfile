#!/usr/bin/env just --justfile

install_deps:
    cargo install sqlx-cli

run:
    cargo run --color=always --package platform-rs --bin platform-rs

test:
    docker compose up -d db
    @echo "Let's give the DB a chance to start up..." && sleep 5
    sqlx migrate run --database-url postgresql://postgres:password@localhost:5432/postgres
    cargo test --bin platform-rs

reset_db:
	docker compose down --volumes
