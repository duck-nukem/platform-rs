# Installation

Use the [justfile](https://github.com/casey/just) to perform certain operations

https://cheatography.com/linux-china/cheat-sheets/justfile/

`just --list`

# TODOs

in no particular order

- [ ] Production-like sessions (redis/db)
- [ ] i18n in templates
- [ ] ability to refer to routes via variables not magic strings
- [ ] route guards (e.g. -> 401 redirect to login)
- [ ] JS Nonce for security
- [ ] Abstractions!!
- [x] Add DB migrations
- [x] Testing approach
- [x] env variables
- ~~Reload templates without server restart -> handlebars~~

These seem related?
- [ ] Performance monitoring (CPU/Mem/DB - maybe Docker + Grafana?)
- [x] Postgres?
- [x] Create sqlite db in github actions (maybe Docker + Postgres?)

# Project aims

- No javascript (or as minimal as possible)
- Full SSR
- Browser native only (alerts, HTML5 elements, etc)
- Max performance & minimum resource requirements