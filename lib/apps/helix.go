//go:build linux || darwin || solaris

package apps

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"syscall"

	goValidator "github.com/go-playground/validator/v10"
	gops "github.com/mitchellh/go-ps"
	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/operation"
)

type HelixConfigSlice struct {
	Helix *helixConfig `toml:"helix"`
}

type helixConfig struct {
	Disabled bool   `toml:"disabled"`
	Dark     string `toml:"dark" validate:"required_with=Light"`
	Light    string `toml:"light" validate:"required_with=Dark"`
}

type Helix struct{}

func NewHelix() *Helix {
	return &Helix{}
}

var _ Switchable = (*Helix)(nil)

func (h *Helix) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) goValidator.ValidationErrors {
	if config.Helix == nil {
		return nil
	}

	err := validator.StructCtx(ctx, config.Helix)
	var errs goValidator.ValidationErrors
	if errors.As(err, &errs) {
		return errs
	}

	return nil
}

func (h *Helix) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	// 1) Read, modify, and write the helix config to change the theme for new sessions.
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return fmt.Errorf("unable to get user's home directory: %v", err)
	}

	var themeConfig *helixConfig = config.Helix
	if themeConfig == nil {
		themeConfig = &helixConfig{
			Dark:  "solarized_dark",
			Light: "solarized_light",
		}
	}
	var themeName = themeConfig.Dark
	if mode == operation.LightMode {
		themeName = themeConfig.Light
	}

	// Helix config is consistent by default, but may be provided via CLI flags.
	// https://docs.helix-editor.com/configuration.html
	// TODO: Make this configurable in thcon.toml.
	configPath := filepath.Join(homeDir, ".config", "helix", "config.toml")
	hxConfigBytes, err := os.ReadFile(configPath)
	if err != nil {
		return fmt.Errorf("unable to read helix config: %v", err)
	}

	themeLine := fmt.Sprintf(`theme = "%s"`, themeName)
	themeLineRE := regexp.MustCompile(`(?m)^\s*theme\s*=\s*".+"$`)
	newConfig := themeLineRE.ReplaceAll(hxConfigBytes, []byte(themeLine))
	if err := os.WriteFile(configPath, newConfig, os.ModePerm); err != nil {
		return fmt.Errorf("unable to write new helix config: %v", err)
	}

	// 2) Then send USR1 to all helix instances to force them to reload their config.
	procs, err := gops.Processes()
	if err != nil {
		return fmt.Errorf("unable to list processes: %v", err)
	}

	errs := []error{}
	for _, p := range procs {
		if p.Executable() == "hx" {
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
		return fmt.Errorf("multiple errors detected: %+v", errs)
	}

	return nil
}

func (h *Helix) Name() string {
	const name = "helix"
	return name
}
