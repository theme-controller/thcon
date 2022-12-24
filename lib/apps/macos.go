//go:build darwin

package apps

import (
	"context"
	"fmt"
	"os/exec"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/theme-controller/thcon/lib/operation"
)

type MacOSConfig struct {
	MacOS *struct {
		Disabled bool `toml:"disabled"`
	}
}

type MacOS struct{}

var _ Switchable = (*MacOS)(nil)

func NewMacOS() Switchable {
	return &MacOS{}
}

func (m *MacOS) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) error {
	// No validation necessary, since there's nothing to configure.
	return nil
}

func (m *MacOS) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	const appearanceAppleScriptf = `tell app "System Events" to tell appearance preferences to set dark mode to %t`
	script := fmt.Sprintf(appearanceAppleScriptf, mode == operation.DarkMode)
	cmd := exec.CommandContext(ctx, "osascript", "-e", script)
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("Unable to switch macOS appearance: %+v", err)
	}

	return nil
}

func (m *MacOS) Name() string {
	const name = "macOS Appearance"
	return name
}

func (m *MacOS) Argname() string {
	const argname = "macos"
	return argname
}
