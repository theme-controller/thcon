package apps

import (
	"context"

	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/operation"
)

type Gtk3 struct {
	progress event.ProgressChannel
}

func NewGtk3(progress event.ProgressChannel) Switchable {
	return &Gtk3{
		progress: progress,
	}
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
