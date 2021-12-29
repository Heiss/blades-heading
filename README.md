# Blades-Heading

A transform plugin for [Blades](https://getblades.org) that adds optional attributes to headings. In fact, it implements this [PR](https://github.com/raphlinus/pulldown-cmark/pull/522). Also it adds the class `header` always to heading, so you can style it easier with css or access it with javascript.

## Tutorial

This plugin adds optional attributes to heading. Most times, you will write something like this.

````markdown
# Heading 1
````

But you could write something like this:

````markdown
# Heading 1
# Heading 1 {#id}
# Heading 1 {#id .style-me .style-me2}
# Heading 1 {.style-me}
````

This plugin adds `#id` and `.header` to your markdown file, so the markdown-renderer handles this to identify this heading and set the classes. If you set something, it respects it and uses your value. But there will be always `.header` in your classes.

## Installation

This plugin can be installed as
```bash
cargo install --path .
```

Then, it can be used in Blades as
```toml
[plugins]
transform = ["blades-heading"]
```
