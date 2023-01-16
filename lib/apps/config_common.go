package apps

import (
	"context"
	"encoding/json"
	"errors"
	"os"

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
	dest := new(Config)
	md, err := toml.DecodeFile(configPath, dest)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			return dest, nil
		}
		return nil, err
	}

	log.Debug().
		Interface("undecoded", md.Undecoded()).
		Stringer("decoded", dest).
		Msg("config read")

	return dest, nil
}
