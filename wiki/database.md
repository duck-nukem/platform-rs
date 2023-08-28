### Installation

`cargo install sqlx-cli`

#### Make migration

`sqlx migrate add <model_or_app_name>`

#### Run migration

**Note:** subject to change if a different DB is used

`sqlx migrate run --database-url sqlite://sqlite.db`
