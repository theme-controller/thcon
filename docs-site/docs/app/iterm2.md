# iTerm2

::: warning PLUGIN REQUIRED
[iTerm2](https://iterm2.com) includes light and dark colorschemes by default, but requires [a plugin](https://github.com/theme-controller/thcon-iterm2) and manually-created profiles to interact with `thcon`.
:::

## Usage
1. Install [thcon-iterm2](https://github.com/theme-controller/thcon-iterm2) by downloading its source and running `make install`:

```sh:no-line-numbers
git clone https://github.com/theme-controller/thcon-iterm2.git
cd thcon-iterm2
make install
```

2. If you haven't already, create an iTerm2 profile for light mode and another for dark mode via Preferences > Profiles.

3. In your `thcon.toml`, list the name of the profiles to use in dark mode and light mode:

```toml
[iterm2]
dark = "dark and brooding"
light = "light and jovial"
```

## Config Schema
Section: `iterm2`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | string | The name of the profile to use in dark mode | (none) |
| `light` | string | The name of the profile to use in light mode | (none) |
