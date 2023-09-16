//go:build linux || darwin || freebsd || openbsd || solaris

package apps

import (
	"context"
	"fmt"
	"os"
	"syscall"

	"github.com/mitchellh/go-homedir"
	gops "github.com/mitchellh/go-ps"
	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
)

type KittyConfigSlice struct {
	Kitty *kittyConfig `toml:"kitty"`
}

type kittyConfig struct {
	Dark  string `toml:"dark" validate:"expfile,required_with=Light"`
	Light string `toml:"light" validate:"expfile,required_with=Dark"`
	health.Disabled
}

type Kitty struct{}

var _ Switchable = (*Kitty)(nil)

func (k *Kitty) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.RequiresConfig(ctx, config.Kitty)
}

func (k *Kitty) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	// 1) Replace the ~/.local/state/thcon/kitty-theme.conf symlink
	if config.Kitty == nil {
		return nil
	}
	var themePath = config.Kitty.Dark
	if mode == operation.LightMode {
		themePath = config.Kitty.Light
	}
	themePath, err := homedir.Expand(themePath)
	if err != nil {
		return fmt.Errorf("unable to expand path to config file: %w", err)
	}

	if err := util.ReplaceStateSymlink(themePath, "kitty.conf"); err != nil {
		return err
	}

	// 2) Then send USR1 to all kitty instances to force them to reload their config.
	procs, err := gops.Processes()
	if err != nil {
		return fmt.Errorf("unable to list processes: %w", err)
	}

	errs := []error{}
	for _, p := range procs {
		if p.Executable() == "kitty" {
			osp, err := os.FindProcess(p.Pid())
			if err != nil {
				errs = append(errs, err)
				log.Error().Int("pid", p.Pid()).Err(err).Msg("unable to find process by ID")
				continue
			}
			if err := osp.Signal(syscall.SIGUSR1); err != nil {
				errs = append(errs, err)
				log.Error().Int("pid", p.Pid()).Err(err).Msg("error sending SIGUSR1")
			}
		}
	}
	if len(errs) > 0 {
		return fmt.Errorf("error(s) detected: %+v", errs)
	}

	return nil
}

func (k *Kitty) Name() string {
	const name = "kitty"
	return name
}

func (k *Kitty) Argname() string {
	return k.Name()
}
