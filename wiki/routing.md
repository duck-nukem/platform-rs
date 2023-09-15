# Routing

Define the routes for your module as

```rust
#[derive(Clone, Copy, Debug)]
pub enum MyRoute {
  Foo,
}

impl SerializableAsUrl for MyRoute {
  fn as_url(&self) -> &'static str {
    match self {
      MyRoute::Foo => "/bar"
    } 
  }
}
```

You should now be able to use this in your router as 

```rust
Router::new().route(MyRoute::Foo.as_url(), get(handler));
```

It's also possible to build urls dynamically, so you don't need
to rely on magic strings to refer to routes.

`build_url(Prefix::Root, MyRoute::Foo, QueryParams::None)` will output `/bar`
`build_url(Prefix::Nested("baz"), MyRoute::Foo, QueryParams::None)` will output `/baz/bar`
`build_url(Prefix::Root, MyRoute::Foo, QueryParams::From(vec![("lang", "en")]))` will output `/bar?lang=en`
