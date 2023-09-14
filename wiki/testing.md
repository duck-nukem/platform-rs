# Testing

## What & how to test?

Try to follow a test-first approach. If you're often prototyping because
you're unsure what changes are needed, that should result in a discussion,
and also in some changes that reduce unknowns and uncertainty.

Try to focus testing effort on:

https://www.geepawhill.org/2019/02/18/pro-tip-tdd-focus-on-our-branching-logic/

### Integration tests

Sometimes you'd want to test the whole system.

Currently this can be achieved like:

```rust
#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use sqlx::PgPool;

    use crate::app;
    use crate::authn::models::{Credentials, NewUser};
    use crate::authn::repository::create_user;

    #[sqlx::test] // injects pool: PgPool to tests; uses a test db with isolation
    async fn test_login_handler_should_redirect_if_user_is_not_found(pool: PgPool) {
        // we can init the whole server by providing the pool, and using false for
        // not setting up instrumentation
        let server = TestServer::new(app(pool.clone(), false).await.into_make_service()).unwrap();

        let response = server
            .post("/login")
            .form(&Credentials {
                username: "".into(),
                password: "".into(),
            })
            .await;

        assert_eq!(response.header("Location"), "/login?message=invalid")
    }
}
```

A test function can be found in `src/tests.rs` called `make_server` that can
substitute most of the code for instantiating a test server:

```rust
let server = make_server(pool.clone()).await;
```
