//go:build linux || darwin || freebsd || openbsd || solaris

package apps

import (
	"context"
	"fmt"
	"path/filepath"

	"github.com/mitchellh/go-homedir"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
)

type AlacrittyConfigSlice struct {
	Alacritty *alacrittyConfig `toml:"alacritty"`
}

type alacrittyConfig struct {
	Dark  string `toml:"dark" validate:"expfile,required_with=Light"`
	Light string `toml:"light" validate:"expfile,required_with=Dark"`
	health.Disabled
}

type Alacritty struct{}

var _ Switchable = (*Alacritty)(nil)

func (a *Alacritty) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.RequiresConfig(ctx, config.Alacritty)
}

func (a *Alacritty) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	cfg := config.Alacritty
	if cfg == nil {
		return nil
	}

	themePath := cfg.Dark
	if mode == operation.LightMode {
		themePath = cfg.Light
	}
	useToml := filepath.Ext(themePath) == "toml"

	themePath, err := homedir.Expand(themePath)
	if err != nil {
		return fmt.Errorf("unable to expand path to theme file: %w", err)
	}

	linkName := "alacritty.yml"
	if useToml {
		linkName = "alacritty.toml"
	}

	return util.ReplaceStateSymlink(themePath, linkName)
}

func (a *Alacritty) Name() string {
	const name = "alacritty"
	return name
}

func (a *Alacritty) Argname() string {
	return a.Name()
}
