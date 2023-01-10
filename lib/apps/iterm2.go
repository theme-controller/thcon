//go:build darwin

package apps

import (
	"context"
	"fmt"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/apps/iterm2pb"
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
	if config.Iterm2 == nil {
		return ErrNeedsConfig
	}

	var desiredProfile = config.Iterm2.Dark
	if mode == operation.LightMode {
		desiredProfile = config.Iterm2.Light
	}

	client, err := iterm2pb.NewClient(ctx)
	if err != nil {
		return fmt.Errorf("unable to create iterm2 client: %v", err)
	}

	profiles, err := client.ListProfiles(ctx)
	if err != nil {
		return fmt.Errorf("unable to get profiles: %v", err)
	}

	quotedDesiredProfile := `"` + desiredProfile + `"`
	var foundProfileProps map[string]string

	for _, profile := range profiles {
		props := map[string]string{}

		for _, prop := range profile.Properties {
			k := prop.GetKey()
			v := prop.GetJsonValue()

			if k == "Name" && v != quotedDesiredProfile {
				// This clearly isn't the profile we're looking for,
				// so don't bother gathering more props from it.
				break
			}

			props[k] = v
		}

		// If we never found a "Name" key, move on to the next profile.
		if _, hasName := props["Name"]; !hasName {
			continue
		}

		// Otherwise, it's implied that this is the profile we're looking for.
		log.Debug().Interface("props", props)
		foundProfileProps = props
		break
	}

	profileGuid := foundProfileProps["Guid"]
	// Values in are JSON-encoded strings, so remove the leading and trailing
	// double-quotes.
	profileGuid = profileGuid[1 : len(profileGuid)-1]
	if foundProfileProps == nil {
		return fmt.Errorf("unable to find profile called %q", desiredProfile)
	}

	if err := client.SetDefaultProfile(ctx, profileGuid); err != nil {
		return fmt.Errorf("unable to set default profile to %q: %v", desiredProfile, err)
	}

	sessionIds, err := client.GetSessionIds(ctx)
	if err != nil {
		return fmt.Errorf("unable to list all sessions: %v", err)
	}

	return client.UpdateCurrentProfileInSessions(ctx, foundProfileProps, sessionIds)
}

func (it2 *Iterm2) Name() string {
	const name = "iTerm2"
	return name
}

func (it2 *Iterm2) Argname() string {
	const argname = "iterm2"
	return argname
}
