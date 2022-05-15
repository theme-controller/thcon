package apps

import (
	"context"

	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/operation"
)

type GnomeShellConfig struct {
	GnomeShell *struct {
		Disabled bool   `toml:"disabled"`
		Dark     string `toml:"dark"`
		Light    string `toml:"light"`
	} `toml:"gnome-shell"`
}

type GnomeShell struct{}

var _ Switchable = (*GnomeShell)(nil)

func NewGnomeShell() Switchable {
	return &GnomeShell{}
}

func (g *GnomeShell) Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error {
	gsettings := glib.SettingsNew("org.gnome.shell.extensions.user-theme")
	var theme = "Arc-Dark-solid"
	if mode == operation.LightMode {
		theme = "Arc"
	}
	gsettings.SetString("name", theme)
	glib.SettingsSync()

	return nil
}

func (gt *GnomeShell) Name() string {
	const name = "GNOME Shell User Theme"
	return name
}
