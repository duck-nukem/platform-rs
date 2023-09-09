# Installation

Use the [justfile](https://github.com/casey/just) to perform certain operations

https://cheatography.com/linux-china/cheat-sheets/justfile/

`just --list`

# TODOs

in no particular order

## Core

- [x] Production-like sessions (redis/db)
    - Implemented in [src/session.rs](src/session.rs) but unused - `CookieStore` is used instead. It's a best practice
      to keep session in a database so they can be invalidated. I'm willing to sacrifice on this as the 3rd party
      package doesn't allow to easily assign users to sessions, making this security feature difficult to implement.
      Mass invalidation of sessions is still possible by changing the secret they're signed with.
- [x] i18n in templates
- [x] route guards (e.g. -> 401 redirect to login)
- [x] JS Nonce for security -> see GreetingsTemplate / logged_in.html
- [x] Abstractions!!
- [x] Add DB migrations
- [x] Testing approach
- [x] env variables
- [x] allow locale per ~~view/session~~ user
- ~~Reload templates without server restart -> handlebars~~

## Postgres

- [ ] Performance monitoring (CPU/Mem/DB - maybe Docker + Grafana?)
- [x] Postgres?
- [x] Create sqlite db in GitHub actions (maybe Docker + Postgres?)

## Extras

- [x] test db isolation -> #[sqlx::test]
- [ ] ability to refer to routes via variables not magic strings
- [ ] tie sessions to users in the DB?
- [ ] pre-commit hooks
- [ ] better i18n support (extract strings from .html)
- [x] establish repository pattern and concentrate SQL queries there
- [x] 404 handler

# Performance

- ~~[ ] Replace bcrypt with pbkdf2 for speed?~~ it's even slower

###### 10k users @ 1 thread = ~580mb; avg 3s;

###### 5k users @ 1 thread = ~498mb; avg 3ms;

# Project aims

- No javascript (or as minimal as possible)
- Full SSR
- Browser native only (alerts, HTML5 elements, etc)
- Max performance & minimum resource requirements