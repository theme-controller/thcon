package apps

import (
	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/operation"
)

type Gtk struct{}

func (g *Gtk) Switch(mode operation.Operation, config interface{}) error {
	gsettings := glib.SettingsNew("org.gnome.desktop.interface")
	var theme = "Adwaita-dark"
	if mode == operation.LightMode {
		theme = "adwaita"
	}
	gsettings.SetString("gtk-theme", theme)
	glib.SettingsSync()

	return nil
}

var _ Switchable = (*Gtk)(nil)
