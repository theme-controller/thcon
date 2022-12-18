//go:build linux || dragonfly || freebsd || netbsd || openbsd

package apps

import (
	"context"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/operation"
)

type GnomeShellConfig struct {
	GnomeShell *struct {
		Disabled bool   `toml:"disabled"`
		Dark     string `toml:"dark" validate:"required_with=Light"`
		Light    string `toml:"light" validate:"required_with=Dark"`
	} `toml:"gnome-shell"`
}

type GnomeShell struct{}

var _ Switchable = (*GnomeShell)(nil)

func NewGnomeShell() Switchable {
	return &GnomeShell{}
}

func (g *GnomeShell) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) error {
	if config.GnomeShell == nil {
		return ErrNeedsConfig
	}

	return validator.StructCtx(ctx, config.GnomeShell)
}

func (g *GnomeShell) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
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
