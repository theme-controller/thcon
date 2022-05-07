package config

import (
	"context"
	"encoding/json"

	"github.com/BurntSushi/toml"
	"github.com/apex/log"
)

func (c Config) String() string {
	marshalled, err := json.Marshal(c)
	if err != nil {
		return ""
	}
	return string(marshalled)
}

func Parse(ctx context.Context) (*Config, error) {
	logger := log.FromContext(ctx)
	configPath, err := ConfigFilePath()
	if err != nil {
		return nil, err
	}

	logger.WithField("path", configPath).Info("reading config")
	var dest Config
	md, err := toml.DecodeFile(configPath, &dest)
	if err != nil {
		return nil, err
	}
	logger.
		WithField("undecoded", md.Undecoded()).
		WithField("decoded", dest).
		Debug("config read")
	return &dest, nil
}
