---
title: Sample
layout: layouts/main.html
eleventyNavigation:
  key: sample
  parent: root
  title: Sample
---

# Sample Configuration
Config files must be in `~/.config/thcon/thcon.toml`. Currently, only one config
file is supported, but apps unsupported on the current platform are ignored. In
other words, it's safe to share a single config file across your macOS and
Ubuntu machines, for example.

This config simply is a collection of the examples listed for each app-specific
page.

```toml
# ~/.config/thcon/thcon.toml

{% for app in collections.app -%}
  {{ app.data.example | safe }}
{% endfor -%}
```
