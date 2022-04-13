package apps

import (
	"context"

	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/operation"
)

type Gtk struct {
	progress event.ProgressChannel
}

func NewGtk(progress event.ProgressChannel) Switchable {
	return &Gtk{
		progress: progress,
	}
}

func (g *Gtk) Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error {
	gsettings := glib.SettingsNew("org.gnome.desktop.interface")
	var theme = "Adwaita-dark"
	if mode == operation.LightMode {
		theme = "adwaita"
	}
	gsettings.SetString("gtk-theme", theme)
	glib.SettingsSync()

	return nil
}

func (gt *Gtk) Name() string {
	const name = "GTK"
	return name
}

var _ Switchable = (*Gtk)(nil)
