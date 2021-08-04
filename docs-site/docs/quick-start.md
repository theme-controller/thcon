# Quick Start
## Installation
For all platforms, please install Rust following [the recommended instructions](https://www.rust-lang.org/learn/get-started).

### macOS
```bash:no-line-numbers
cargo install thcon
```

### Ubuntu 20.04 (Focal)
```bash:no-line-numbers
sudo apt install build-essential libdbus-1-dev libglib2.0-dev
cargo install thcon
```

### Fedora 34
```bash:no-line-numbers
sudo dnf install dbus-devel glib2-devel
cargo install thcon
```

### openSUSE Leap (15.3)
```bash:no-line-numbers
sudo zypper install gcc dbus-1-devel glib2-devel
cargo install thcon
```

## Usage
After installation, `thcon` should work right away for apps that don't require configuration - typically OS-level settings.

### Switch All Apps
All applications can be switched to dark mode with one command:

```bash:no-line-numbers
thcon dark
```

or back to light mode with:

```bash:no-line-numbers
thcon light
```

### Switch Specific Apps
If you only want to switch specific applications, list those apps after `dark` or `light`:

<CodeGroup>
<CodeGroupItem title="macOS">

```bash:no-line-numbers
thcon dark macos
```

</CodeGroupItem>
<CodeGroupItem title="KDE">

```bash:no-line-numbers
thcon dark plasma
```

</CodeGroupItem>
<CodeGroupItem title="GTK">

```bash:no-line-numbers
thcon dark gtk
```

</CodeGroupItem>
</CodeGroup>

## Configuration
Individual apps are configured via the `thcon.toml` file, which lives in one of these locations (depending on your operating system):

* Windows: TBD
* macOS, Linux, BSD: `~/.config/thcon/thcon.toml`

See each app's documentation for the specific configuration options available.
