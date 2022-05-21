package util

import (
	"errors"
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
		return "", fmt.Errorf("Unable to create thcon state directory for current user: %+v", err)
	}

	return dirname, nil
}

// SymlinkExists returns (true, nil) if the symlink at 'filename' exists,
// (false, nil) if it doesn't, and (false, error) otherwise.
func SymlinkExists(filename string) (bool, error) {
	_, err := os.Readlink(filename)
	if err == nil {
		return true, nil
	}
	if errors.Is(err, os.ErrNotExist) {
		return false, nil
	}
	return false, err
}
