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

	EmacsConfigSlice
	VimConfigSlice
	NeovimConfigSlice
	HelixConfigSlice
	KittyConfigSlice
	AlacrittyConfigSlice
	ExtensionsSlice
}

func ConfigFilePath() (string, error) {
	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}

	return filepath.Join(configDir, "thcon", "thcon.toml"), nil
}
