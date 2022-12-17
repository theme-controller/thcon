//go:build darwin

package apps

import (
	"context"
	"fmt"
	"os"
	"os/exec"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/operation"
)

type TerminalDotAppConfig struct {
	TerminalDotApp *struct {
		Disabled bool   `toml:"disabled"`
		Dark     string `toml:"dark" validate:"required"`
		Light    string `toml:"light" validate:"required"`
	}
}

type TerminalDotApp struct{}

var _ Switchable = (*TerminalDotApp)(nil)

func (tda *TerminalDotApp) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) goValidator.ValidationErrors {
	return nil
}

func (tda *TerminalDotApp) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	const switchProfileAppleScriptf = `tell application "Terminal"
	set new_settings to first settings set whose name is "%s"

	set default settings to new_settings
	set startup settings to new_settings

	set current settings of every tab of every window to new_settings
end tell`

	var profile string = "Pro"
	if mode == operation.LightMode {
		profile = "Novel"
	}
	script := fmt.Sprintf(switchProfileAppleScriptf, profile)
	log.Debug().Str("script", script).Msg("calling osascript...")
	cmd := exec.CommandContext(ctx, "osascript", "-e", script)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("unable to switch Terminal.app profiles: %+v", err)
	}

	return nil
}

func (tda *TerminalDotApp) Name() string {
	const name = "Terminal.app"
	return name
}
