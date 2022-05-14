package apps

import (
	"context"

	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/operation"
)

type Gtk3 struct{}

func NewGtk3() Switchable {
	return &Gtk3{}
}

func (g *Gtk3) Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error {
	gsettings := glib.SettingsNew("org.gnome.desktop.interface")
	var theme = "Adwaita-dark"
	if mode == operation.LightMode {
		theme = "adwaita"
	}
	gsettings.SetString("gtk-theme", theme)
	glib.SettingsSync()

	return nil
}

func (gt *Gtk3) Name() string {
	const name = "GTK3"
	return name
}

var _ Switchable = (*Gtk3)(nil)
