# Terminal.app

## Usage
[Terminal.app](https://support.apple.com/guide/terminal/welcome/mac)'s default "Basic" profile has is aware of the macOS dark mode setting, and will react accordingly.  No other themes appear to behave that way, however.  For manually-added (or imported) profiles, simply list the names of the desired light mode and dark mode profiles in your `thcon.toml`.

```toml
[terminal-app]
dark = "Pro"
light = "Silver Aerogel"
```

## Config Schema
Section: `terminal-app`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | string | The name of the profile to use in dark mode | Pro |
| `light` | string | The name of the profile to use in light mode | Basic |
