//go:build linux || dragonfly || freebsd || netbsd || openbsd

package apps

import (
	"context"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/operation"
)

type Gtk3Config struct {
	Gtk3 *struct {
		Disabled bool   `toml:"disabled"`
		Dark     string `toml:"dark" validate:"required_with=Light"`
		Light    string `toml:"light" validate:"required_with=Dark"`
	} `toml:"gtk3"`
}

type Gtk3 struct{}

func NewGtk3() Switchable {
	return &Gtk3{}
}

func (g *Gtk3) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) error {
	if config.Gtk3 == nil {
		return nil
	}

	return validator.StructCtx(ctx, config.Gtk3)
}

func (g *Gtk3) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
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

func (gt *Gtk3) Argname() string {
	const argname = "gtk3"
	return argname
}

var _ Switchable = (*Gtk3)(nil)
