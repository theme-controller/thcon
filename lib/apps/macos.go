//go:build darwin

package apps

import (
	"context"
	"fmt"
	"os/exec"

	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
)

type MacOSConfigSlice struct {
	MacOS *macOSConfig `toml:"macos"`
}

type macOSConfig struct {
	health.Disabled
}

type MacOS struct{}

var _ Switchable = (*MacOS)(nil)

func (m *MacOS) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.HasDefaults(ctx, config.MacOS)
}

func (m *MacOS) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	const appearanceAppleScriptf = `tell application "System Events" to tell appearance preferences to set dark mode to %t`
	script := fmt.Sprintf(appearanceAppleScriptf, mode == operation.DarkMode)
	cmd := exec.CommandContext(ctx, "osascript", "-e", script)
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("unable to switch macOS appearance: %+v", err)
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
