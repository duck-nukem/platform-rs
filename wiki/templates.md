Refer to the [Sailfish docs](https://rust-sailfish.github.io/sailfish/syntax/overview/)

### Summary
* `<? ?>`: Inline tag, you can write Rust code inside this tag
* `<?= ?>`: Evaluate the Rust expression and outputs the value into the template (HTML escaped)
* `<?- ?>`: Evaluate the Rust expression and outputs the unescaped value into the template
* `<?+ ?>`: Evaluate the Rust expression producing a TemplateOnce value, and render that value into the template
* `<?# ?>`: Comment tag
* `<??`: Outputs a literal '<?'

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