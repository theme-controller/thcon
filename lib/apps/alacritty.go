//go:build linux || darwin || freebsd || openbsd || solaris

package apps

import (
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"syscall"

	"github.com/mitchellh/go-homedir"
	gops "github.com/mitchellh/go-ps"
	"github.com/rs/zerolog/log"
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
	// Just copy the dark file into ~/.local/share/thcon/alacritty.yaml
	// and have users use that as an import
}

func (a *Alacritty) Name() string {
	const name = "alacritty"
	return name
}

func (a *Alacritty) Argname() string {
	return a.Name()
}
