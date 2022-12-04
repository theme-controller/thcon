package config

import (
	"os"
	"path/filepath"

	"github.com/theme-controller/thcon/lib/apps"
)

type Config struct {
	apps.AllVimConfig
}

func ConfigFilePath() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(homeDir, ".config", "thcon", "thcon.toml"), nil
}
