package util

import (
	"fmt"
	"os"
	"path/filepath"
)

func EnsureThconStateDir() (string, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}

	dirname := filepath.Join(home, ".local", "share", "thcon")
	err = os.MkdirAll(dirname, 0700)
	if err != nil {
		return "", fmt.Errorf("unable to create thcon state directory for current user: %+w", err)
	}

	return dirname, nil
}
