# Installation

Use the [justfile](https://github.com/casey/just) to perform certain operations

https://cheatography.com/linux-china/cheat-sheets/justfile/

`just --list`

# Project aims

- No javascript (or as minimal as possible)
- Secure!
- Full SSR, optinally with HTMX
- Browser native only (alerts, HTML5 elements, etc)
- Max performance & minimum resource requirements

Last recorded performance: 

* ~1k rps with sessions in postgres 
* ~6k rps with only cookie-based sessions