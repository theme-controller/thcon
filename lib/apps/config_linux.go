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

	VimConfigSlice
	NeovimConfigSlice
	HelixConfigSlice
}

func ConfigFilePath() (string, error) {
	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}

	return filepath.Join(configDir, "thcon", "thcon.toml"), nil
}
