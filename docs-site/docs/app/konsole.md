# Konsole

## Usage: Linux & BSD
Konsole instances can be discovered and controlled via DBus, but it's a cumbersome process to perform in a one-liner. `thcon` simplifies that - just list the name of the Konsole profiles you prefer in light mode and in dark mode in your `thcon.toml`, e.g.:

```toml
[plasma]
dark = "Some dark profile"
light = "A light profile"
```

## Usage: Windows & macOS
Konsole is not available on Windows or macOS.

## Config Schema
Section: `konsole`

| Key | Type | Description | Default |
| --- | ---- | ----------- | -------- |
| `disabled` | boolean | `true` to disable theming of this app, otherwise `false` | `false` |
| `dark` | string | The name of the profile (case-sensitive) to use in dark mode | (none) |
| `light` | string | The name of the profile (case-sensitive) to use in light mode | (none) |
