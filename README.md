# Installation

Use the [justfile](https://github.com/casey/just) to perform certain operations

https://cheatography.com/linux-china/cheat-sheets/justfile/

`just --list`

# TODOs

in no particular order

## Core

- [x] Production-like sessions (redis/db)
- [x] i18n in templates
- [x] route guards (e.g. -> 401 redirect to login)
- [x] JS Nonce for security -> see GreetingsTemplate / logged_in.html
- [x] Abstractions!!
- [x] Add DB migrations
- [x] Testing approach
- [x] env variables
- [x] allow locale per ~~view/session~~ user

## Postgres

- [x] Performance monitoring (CPU/Mem/DB - maybe Docker + Grafana?)
- [x] Postgres?
- [x] Create sqlite db in GitHub actions (maybe Docker + Postgres?)

## Extras

- [x] test db isolation -> #[sqlx::test]
- [x] pre-commit hooks
- [x] establish repository pattern and concentrate SQL queries there
- [x] 404 handler
- [ ] find an alternative auth mechanism that doesn't need DB storage
- [ ] better i18n support (extract strings from .html)
- [ ] "Dev mode" - incremental recompilation on changes for faster prototyping
- [ ] ability (/pattern?) to refer to routes via variables not magic strings
- [ ] admin-like screens for managing entities?

## Security

- [ ] csrf token impl
- [x] secrets from envvars

# Project aims

- No javascript (or as minimal as possible)
- Secure!
- Full SSR, optinally with HTMX
- Browser native only (alerts, HTML5 elements, etc)
- Max performance & minimum resource requirements

Last recorded performance: 

* ~1k rps with sessions in postgres 
* ~6k rps with only cookie-based sessions