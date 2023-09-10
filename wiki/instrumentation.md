# Instrumentation

## Current setup

Using jaeger @ http://localhost:16686

## User guide

To trace something add the

```rust
# [instrument]
```

on top of the function definition, like so:

```rust
#[instrument]
pub async fn login_view(Query(params): Query<Params>) -> Html<String> {
    Ok(())
}
```

`#[instrument]` will create a trace in jaeger (http://localhost:16686).

If you'd like to see the result of the function, you should use
`#[instrument(ret)]` - but be careful as this might expose sensitive data!

Calling `tracing::info!("message")` will append the data to tracing.

It's recommended to decorate all repository functions (doing database queries) and all views that don't contain
sensitive data.