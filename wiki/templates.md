Refer to the [Sailfish docs](https://rust-sailfish.github.io/sailfish/syntax/overview/)

### Summary
* `<? ?>`: Inline tag, you can write Rust code inside this tag
* `<?= ?>`: Evaluate the Rust expression and outputs the value into the template (HTML escaped)
* `<?- ?>`: Evaluate the Rust expression and outputs the unescaped value into the template
* `<?+ ?>`: Evaluate the Rust expression producing a TemplateOnce value, and render that value into the template
* `<?# ?>`: Comment tag
* `<??`: Outputs a literal '<?'

### Javascript in Templates

At the time of writing this, the CSP is set to only allow
scripts with a nonce. This should mitigate most XSS attacks.

> Learn more about nonce CSP at this [presentation](https://speakerdeck.com/mikispag/the-web-is-broken-lets-fix-it-97d7b73d-516d-4709-90e1-837f5c3d5fa2?slide=44)

The nonce is currently set to the request id
- it's random enough that it can't be easily guessed
- it's available in view handlers (so they can be passed to templates) and in middlewares (so CSP headers can be set)
- it's unique for each request

To be able to run scripts, you can either use 3rd party ones

```html
<script src="unpkg.com/test.js" nonce="<?- nonce ?>"></script>
```

or inlines

```html
<script nonce="<?- nonce ?>">alert("CSP Works!");</script>
```

the important bit is that the nonce is defined.

To get the request id from the request and pass it to the templates you should
be able to do something like 


```rust
use crate::templates::render;

use axum::body::Body;
use axum::http::Request;
use axum::response::{IntoResponse};

pub async fn example_view_handler(req: Request<Body>) -> impl IntoResponse {
  let request_id = &*req.extensions().get::<RequestId>().map(ToString::to_string).unwrap();
  render(MyTemplate { nonce: request_id.to_string() })
}

#[derive(TemplateOnce)]
#[template(path = "my_template.html")]
struct MyTemplate {
  nonce: String,
}
```

### Common examples

#### Condition

```html
<? if messages.is_empty() { ?>
  <div>No messages</div>
<? } ?>
```

#### Loop

```html
<? for (i, msg) in messages.iter().enumerate() { ?>
  <div><?= i ?>: <?= msg ?></div>
<? } ?>
```

#### Includes

```html
<? include!("path/to/template"); ?>
```

#### Filters

```html
<?= message | upper ?>
```

```json
{
    "id": <?= id ?>
    "comment": <?- comment | json ?>
}
```