package apps

import (
	"os"
	"path/filepath"
)

type Config struct {
	TerminalDotAppConfigSlice
	VimConfigSlice
	NeovimConfigSlice
	HelixConfigSlice
}

func ConfigFilePath() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(homeDir, ".config", "thcon", "thcon.toml"), nil
}
