package apps

import (
	"context"
	"encoding/json"

	"github.com/BurntSushi/toml"
	"github.com/rs/zerolog/log"
)

func (c Config) String() string {
	marshalled, err := json.Marshal(c)
	if err != nil {
		return ""
	}
	return string(marshalled)
}

func ParseConfig(ctx context.Context, configPath string) (*Config, error) {
	log.Info().Str("path", configPath).Msg("reading config")
	var dest Config
	md, err := toml.DecodeFile(configPath, &dest)
	if err != nil {
		return nil, err
	}

	log.Debug().
		Interface("undecoded", md.Undecoded()).
		Stringer("decoded", dest).
		Msg("config read")

	return &dest, nil
}
