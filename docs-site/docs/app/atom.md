---
title: Atom
---

# Atom

::: warning PLUGIN REQUIRED
[Atom](https://atom.io) includes a light and dark theme by default, but requires [a plugin](https://github.com/theme-controller/thcon-atom) to interact with `thcon`.
:::

Switches between [Atom](https://atom.io) UI and editor themes in all windows and tabs.

## Usage
First, ensure you've installed [thcon-atom](https://github.com/theme-controller/thcon-atom):

```sh:no-line-numbers
apm install thcon
```

If you like the default light and dark UI and syntax themes, you're done!  If you prefer other color schemes, you'll need to add those themes in `thcon.toml`.  These can be copy-pasted from the `core.themes` property in your `config.cson`.  Simply get Atom looking right in dark mode, copy those themes into `thcon.toml`, then repeat for light mode.

```toml
[atom]
dark = [ "one-dark-ui", "one-dark-syntax" ]
light = [ "one-light-ui", "one-light-syntax" ]
```

## Config Schema
Section: `atom`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | array of strings | The themes to apply in dark mode, as shown in `config.cson` | `["one-dark-ui", "one-dark-syntax"]` |
| `light` | array of strings | The themes to apply in dark mode, as shown in `config.cson` | `["one-light-ui", "one-light-syntax"]` |
