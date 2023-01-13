//go:build darwin

package apps

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
)

type Iterm2ConfigSlice struct {
	Iterm2 *iterm2Config `toml:"iterm2"`
}

type iterm2Config struct {
	health.Disabled
	Dark  string `toml:"dark" validate:"required"`
	Light string `toml:"light" validate:"required"`
}

type Iterm2 struct{}

var _ Switchable = (*Iterm2)(nil)

func (it2 *Iterm2) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.RequiresConfig(ctx, config.Iterm2)
}

func (it2 *Iterm2) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	if config.Iterm2 == nil {
		return nil
	}

	var desiredProfile string = config.Iterm2.Dark
	if mode == operation.LightMode {
		desiredProfile = config.Iterm2.Light
	}
	desiredProfileJson, err := json.Marshal(desiredProfile)
	if err != nil {
		return fmt.Errorf("unable to marshal theme %q as JSON: %v", desiredProfile, err)
	}
	desiredProfile = string(desiredProfileJson)

	confdir, err := os.UserConfigDir()
	if err != nil {
		return fmt.Errorf("unable to get user config directory: %v", err)
	}

	// See https://iterm2.com/documentation-dynamic-profiles.html
	dyProfilePath := filepath.Join(confdir, "iTerm2", "DynamicProfiles", "thcon.json")
	f, err := os.OpenFile(dyProfilePath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0o600)
	if err != nil {
		return fmt.Errorf("unable to open dynamic profile %q: %v", dyProfilePath, err)
	}
	defer f.Close()

	_, err = fmt.Fprintf(f, strings.TrimSpace(`
{
  "Profiles": [
    {
      "Name": "thcon",
      "Guid": "4379c5b2-2a9a-48f7-a73f-e569fa1a35b9",
      "Dynamic Profile Parent Name": %s
    }
  ]
}`), desiredProfile)

	if err != nil {
		return fmt.Errorf("unable to write dynamic profile %q: %v", dyProfilePath, err)
	}
	return nil
}

func (it2 *Iterm2) Name() string {
	const name = "iTerm2"
	return name
}

func (it2 *Iterm2) Argname() string {
	const argname = "iterm2"
	return argname
}
