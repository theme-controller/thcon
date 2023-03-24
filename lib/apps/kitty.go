//go:build linux || darwin || freebsd || openbsd || solaris

package apps

import (
	"context"
	"fmt"
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

type KittyConfigSlice struct {
	Kitty *kittyConfig `toml:"kitty"`
}

type kittyConfig struct {
	Dark  string `toml:"dark" validate:"required_with=Light"`
	Light string `toml:"light" validate:"required_with=Dark"`
	health.Disabled
}

type Kitty struct{}

func NewKitty() *Kitty {
	return &Kitty{}
}

var _ Switchable = (*Kitty)(nil)

func (k *Kitty) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.HasDefaults(ctx, config.Kitty)
}

func (k *Kitty) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	// 1) Replace the ~/.local/state/thcon/kitty-theme.conf symlink
	var themeConfig *kittyConfig = config.Kitty
	if themeConfig == nil {
		themeConfig = &kittyConfig{
			Dark:  "~/.config/kitty/dark.thcon.conf",
			Light: "~/.config/kitty/light.thcon.conf",
		}
	}
	var themePath = themeConfig.Dark
	if mode == operation.LightMode {
		themePath = themeConfig.Light
	}
	themePath, err := homedir.Expand(themePath)
	if err != nil {
		return fmt.Errorf("unable to expand path to config file: %v", err)
	}

	stateDir, err := util.EnsureThconStateDir()
	if err != nil {
		return err
	}

	symlinkTarget := filepath.Join(stateDir, "kitty.conf")
	exists, err := util.SymlinkExists(symlinkTarget)
	if err != nil {
		return err
	}

	if exists {
		if err := os.Remove(symlinkTarget); err != nil {
			return fmt.Errorf("unable to remove old symlink: %v", err)
		}
	}
	if err := os.Symlink(themePath, symlinkTarget); err != nil {
		return fmt.Errorf("unable to create symlink for theme: %v", err)
	}

	// 2) Then send USR1 to all kitty instances to force them to reload their config.
	procs, err := gops.Processes()
	if err != nil {
		return fmt.Errorf("unable to list processes: %v", err)
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
