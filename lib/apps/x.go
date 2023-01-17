package apps

import (
	"context"
	"fmt"
	"os"
	"os/exec"

	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
)

type ExtensionsSlice struct {
	Extensions map[string]*extensionConfig `toml:"x"`
}

func MakeExtensions(ctx context.Context, config *Config) []Switchable {
	out := []Switchable{}
	if config.Extensions == nil {
		return out
	}

	for key, xConfig := range config.Extensions {
		out = append(out, &Extension{key: key, config: xConfig})
	}

	return out
}

type extensionConfig struct {
	Name  string `toml:"name" validate:"required"`
	Dark  string `toml:"dark" validate:"required_with=Light"`
	Light string `toml:"light" validate:"required_with=Dark"`
	health.Disabled
}

type Extension struct {
	key    string
	config *extensionConfig
}

var _ Switchable = (*Extension)(nil)

func (x *Extension) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.RequiresConfig(ctx, x.config)
}

func (x *Extension) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	var switchCmd string = x.config.Dark
	if mode == operation.LightMode {
		switchCmd = x.config.Light
	}

	argv := append(
		[]string{"-c"},
		switchCmd,
	)
	// TODO: make this work in windows.
	// Doing this "right" probably requires shellescape or something?
	cmd := exec.CommandContext(ctx, "sh", argv...)

	cmd.Env = append(
		os.Environ(),
		"THCON_MODE="+mode.String(),
	)

	out, err := cmd.CombinedOutput()
	if err != nil {
		log.Ctx(ctx).Error().Bytes("output", out).Msg("extension error")
		return fmt.Errorf("error executing custom comand %q: %v", switchCmd, err)
	}

	if len(out) > 0 {
		log.Ctx(ctx).Debug().Bytes("output", out).Msg("extension output")
	}
	return nil
}

func (x *Extension) Name() string {
	return x.config.Name
}

func (x *Extension) Argname() string {
	return x.key
}
