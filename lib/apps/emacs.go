package apps

import (
	"context"
	"fmt"

	"github.com/mitchellh/go-homedir"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
)

type EmacsConfigSlice struct {
	Emacs *emacsConfig `toml:"emacs"`
}

type emacsConfig struct {
	Dark  string `toml:"dark" validate:"expfile,required_with=Light"`
	Light string `toml:"light" validate:"expfile,required_with=Dark"`
	health.Disabled
}

type Emacs struct{}

var _ Switchable = (*Emacs)(nil)

func (*Emacs) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.RequiresConfig(ctx, config.Emacs)
}

func (*Emacs) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	cfg := config.Emacs
	if cfg == nil {
		return nil
	}

	themePath := cfg.Dark
	if mode == operation.LightMode {
		themePath = cfg.Light
	}

	themePath, err := homedir.Expand(themePath)
	if err != nil {
		return fmt.Errorf("unable to expand path to theme file: %w", err)
	}

	return util.CopyToStateDir(themePath, "emacs.el")
}

func (e *Emacs) Argname() string {
	return e.Name()
}

func (*Emacs) Name() string {
	const name = "emacs"
	return name
}
