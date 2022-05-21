package config

import (
	"os"
	"path/filepath"

	"github.com/theme-controller/thcon/lib/apps"
)

type Config struct{
	apps.GnomeTerminalConfig
	apps.GnomeShellConfig
	apps.Gtk3Config

	apps.KonsoleConfig
	apps.PlasmaConfig

	apps.AllVimConfig
}

func ConfigFilePath() (string, error) {
	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}

	return filepath.Join(configDir, "thcon", "thcon.toml"), nil
}
