//go:build darwin

package apps

import (
	"context"
	"fmt"
	"os"
	"os/exec"

	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
)

type TerminalDotAppConfigSlice struct {
	TerminalDotApp *terminalDotAppConfig `toml:"terminal-app"`
}

type terminalDotAppConfig struct {
	Dark  string `toml:"dark" validate:"required_with=Light"`
	Light string `toml:"light" validate:"required_with=Dark"`
	health.Disabled
}

type TerminalDotApp struct{}

var _ Switchable = (*TerminalDotApp)(nil)

func (tda *TerminalDotApp) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.HasDefaults(ctx, config.TerminalDotApp)
}

func (tda *TerminalDotApp) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	const switchProfileAppleScriptf = `tell application "Terminal"
	set was_running to running
	set new_settings to first settings set whose name is "%s"

	set default settings to new_settings
	set startup settings to new_settings

	set current settings of every tab of every window to new_settings

	# Quit Terminal.app if it wasn't already running (set ... above starts it in
	# the background).
	if not was_running then do shell script "pkill -x Terminal"
end tell`

	var themeConfig *terminalDotAppConfig = config.TerminalDotApp
	shouldApplyDefaults := themeConfig == nil || (!themeConfig.IsDisabled() && themeConfig.Light == "" && themeConfig.Dark == "")
	if shouldApplyDefaults {
		themeConfig = &terminalDotAppConfig{
			Dark:  "Pro",
			Light: "Basic",
		}
	}
	var profile string = themeConfig.Dark
	if mode == operation.LightMode {
		profile = themeConfig.Light
	}

	script := fmt.Sprintf(switchProfileAppleScriptf, profile)
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

func (tda *TerminalDotApp) Argname() string {
	const argname = "terminal-app"
	return argname
}
