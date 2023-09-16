//go:build linux || dragonfly || freebsd || netbsd || openbsd

package apps

import (
	"context"
	"errors"
	"os/exec"

	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
)

type PlasmaConfig struct {
	Plasma *struct {
		Disabled bool   `toml:"disabled"`
		Dark     string `toml:"dark" validate:"required_with=Light"`
		Light    string `toml:"light" validate:"required_with=Dark"`
	} `toml:"plasma"`
}

type Plasma struct{}

func (p *Plasma) Name() string {
	const name = "Plasma"
	return name
}

func (p *Plasma) Argname() string {
	const argname = "plasma"
	return argname
}

func (p *Plasma) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	_, err := exec.LookPath("lookandfeeltool")
	if err != nil {
		return health.StatusNotInstalled, errors.New("lookandfeeltool is not installed")
	}
	return health.HasDefaults(ctx, config.Plasma)
}

func (p *Plasma) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	var packageName string
	switch mode {
	case operation.DarkMode:
		packageName = "org.kde.breezedark.desktop"
	case operation.LightMode:
		packageName = "org.kde.breeze.desktop"
	}

	laft := exec.CommandContext(
		ctx,
		"lookandfeeltool",
		"--apply",
		packageName,
	)
	err := laft.Run()
	if err != nil {
		log.Error().
			Err(err).
			Msg("exec lookandfeeltool")
		return err
	}

	return nil
}
