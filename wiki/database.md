# Installation

`cargo install sqlx-cli`

## Make migrations

`sqlx migrate add <model_or_app_name>`

## Run migration

**Note:** subject to change if a different DB is used

`sqlx migrate run --database-url sqlite://sqlite.db`

## User guide

sqlx will map results to your struct, but if there's a mismatch, it'll fail as if there were no rows found!

For example:

````rust
struct MyModel {
    id: String,
    language: String,
}
````

If you do a query  `SELECT id FROM users` it won't have `language` thus won't be able to serialize this object.

The same applies if `language` is `NULL` in the DB, but your struct isn't an `Option<String>`!

### Pooling

Try to use `Extension(pool): Extension<PgPool>` in views/handlers rather than creating a connection
every time you need one (via `get_pool()`). Manually creating multiple connections can result in
postgres running out of pool connections to handle, making the app fail.