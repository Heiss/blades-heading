A transform plugin for [Blades](https://getblades.org) that adds optional attributes to headings. In fact, it implements this [PR](https://github.com/raphlinus/pulldown-cmark/pull/522). Also it adds the class `header` always to heading, so you can style it easier with css or access it with javascript.

## Tutorial

General use:

````markdown
# Heading 1
````

Example:
````markdown
# Heading 1
# Heading 1 {#id}
# Heading 1 {#id .style-me}
# Heading 1 {.style-me}
````

This plugin can be installed as
```bash
cargo install --path .
```

Then, it can be used in Blades as
```toml
[plugins]
transform = ["blades-heading"]
```
