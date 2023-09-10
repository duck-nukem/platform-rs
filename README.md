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

- [ ] Performance monitoring (CPU/Mem/DB - maybe Docker + Grafana?)
- [x] Postgres?
- [x] Create sqlite db in GitHub actions (maybe Docker + Postgres?)

## Extras

- [x] test db isolation -> #[sqlx::test]
- [ ] ability (/pattern?) to refer to routes via variables not magic strings
- [x] pre-commit hooks
- [ ] better i18n support (extract strings from .html)
- [x] establish repository pattern and concentrate SQL queries there
- [x] 404 handler
- [ ] "Dev mode" - incremental recompilation on changes for faster prototyping
- [ ] admin-like screens for managing entities?

## Security

- [ ] tie sessions to users in the DB?
    - limited use-case: invalidating all tokens for a given user
    - postponed because the 3rd party library makes it borderline impossible to do this (without reimplementing it)
- [ ] csrf token impl
- [x] secrets from envvars

# Project aims

- No javascript (or as minimal as possible)
- Secure!
- Full SSR, optinally with HTMX
- Browser native only (alerts, HTML5 elements, etc)
- Max performance & minimum resource requirements