//go:build darwin

package apps

import (
	"context"
	"encoding/json"
	"fmt"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/theme-controller/thcon/lib/ipc"
	"github.com/theme-controller/thcon/lib/operation"
)

type Iterm2ConfigSlice struct {
	Iterm2 *iterm2Config `toml:"iterm2"`
}

type iterm2Config struct {
	Disabled bool   `toml:"disabled"`
	Dark     string `toml:"dark" validate:"required"`
	Light    string `toml:"light" validate:"required"`
}

type Iterm2 struct{}

var _ Switchable = (*Iterm2)(nil)

func (it2 *Iterm2) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) error {
	if config.Iterm2 == nil {
		return ErrNeedsConfig
	}

	return validator.StructCtx(ctx, config.Iterm2)
}

func (it2 *Iterm2) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	type it2Payload struct {
		Profile string `json:"profile"`
	}

	if config.Iterm2 == nil {
		return ErrNeedsConfig
	}

	var profile = config.Iterm2.Dark
	if mode == operation.LightMode {
		profile = config.Iterm2.Light
	}

	socks, err := ipc.ListSockets("iterm2", false /* socketPerProcess */)
	if err != nil {
		return fmt.Errorf("unable to list sockets: %v", err)
	}

	// With no sockets found, iTerm2 is likely not running.
	if len(socks) == 0 {
		return nil
	}

	if len(socks) > 1 {
		return fmt.Errorf("expected one socket, but found multiple: %v", socks)
	}

	msg, err := json.Marshal(it2Payload{
		Profile: profile,
	})
	if err != nil {
		return fmt.Errorf("unable to JSON-encode iTerm2 payload: %v", err)
	}

	return ipc.Send(ctx, &ipc.Outbound{
		Socket:  socks[0],
		Message: msg,
	})
}

func (it2 Iterm2) Name() string {
	const name = "iTerm2"
	return name
}
