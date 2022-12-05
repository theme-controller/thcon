//go:build darwin

package apps

import (
	"context"
	"fmt"
	"os/exec"

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

func (m *MacOS) Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error {
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
