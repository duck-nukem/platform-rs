#!/usr/bin/env just --justfile

install_deps:
    cargo install sqlx-cli

run:
    cargo run --color=always --package platform-rs --bin platform-rs

test:
    touch test_sqlite.db
    sqlx migrate run --database-url sqlite://test_sqlite.db
    cargo test