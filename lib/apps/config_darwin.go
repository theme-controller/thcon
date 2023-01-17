package apps

import (
	"os"
	"path/filepath"
)

type Config struct {
	MacOSConfigSlice
	TerminalDotAppConfigSlice
	VimConfigSlice
	NeovimConfigSlice
	HelixConfigSlice
	Iterm2ConfigSlice
	ExtensionsSlice
}

func ConfigFilePath() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(homeDir, ".config", "thcon", "thcon.toml"), nil
}
