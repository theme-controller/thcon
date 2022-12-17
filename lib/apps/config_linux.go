package apps

import (
	"os"
	"path/filepath"
)

type Config struct {
	GnomeTerminalConfig
	GnomeShellConfig
	Gtk3Config

	KonsoleConfig
	PlasmaConfig

	VimConfig
	NeovimConfig
	HelixConfig
}

func ConfigFilePath() (string, error) {
	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}

	return filepath.Join(configDir, "thcon", "thcon.toml"), nil
}
