---
title: Sublime Text 4
---

# Sublime Text 4

::: tip THCON NOT REQUIRED
Sublime Text 4 can automatically synchronize your UI theme and colorscheme with your OS dark mode!  `thcon` isn't involved in this process, but the appropriate configuration is detailed here anyway :heart:
:::

## Preferences.sublime-settings
### Default Themes
In your `Preferences.sublime-settings` (easily accessed via "Preferences > Settings" from the Sublime Text 4 menubar), set `color_scheme` and `theme` to `"auto"` to switch between the default themes:

```json
{
    // ...
    "color_scheme": "auto",
    "theme": "auto"
}
```

### Custom Themes
To define custom colorschemes, set `dark_color_scheme` and `light_color_scheme` in your settings.  To define custom UI themes, set `dark_theme` and `light_theme` in your settings.

```json
{
    // ...
    "color_scheme": "auto",
    "dark_color_scheme": "Packages/Color Scheme - Default/Monokai.sublime-color-scheme",
    "light_color_scheme": "Packages/Color Scheme - Default/Celeste.sublime-color-scheme",

    "theme": "auto"
    "dark_theme": "Default Dark.sublime-theme",
    "light_theme": "Default.sublime-theme"
}
```
