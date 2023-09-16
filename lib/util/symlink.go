package util

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
)

// ReplaceSymlink creates newname in the thcon state directory as a symbolic
// link to oldname, and removes newname first if it already exists.
func ReplaceStateSymlink(oldname, newname string) error {
	stateDir, err := EnsureThconStateDir()
	if err != nil {
		return err
	}

	newpath := filepath.Join(stateDir, newname)
	if err := os.Remove(newpath); err != nil && !errors.Is(err, os.ErrNotExist) {
		return fmt.Errorf("unable to remove old symlink: %w", err)
	}

	if err := os.Symlink(oldname, newpath); err != nil {
		return fmt.Errorf("unable to create symlink: %w", err)
	}

	return nil
}
