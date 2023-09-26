package util

import (
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/rs/zerolog/log"
)

// CopyToStateDir copies the file at srcPath to a file named dstName in the
// thcon state directory, overwriting it if it already exists.
func CopyToStateDir(srcPath, dstName string) error {
	stateDir, err := EnsureThconStateDir()
	if err != nil {
		return err
	}

	srcFile, err := os.Open(srcPath)
	if err != nil {
		return fmt.Errorf("unable to open source file %q: %w", srcPath, err)
	}

	dstPath := filepath.Join(stateDir, dstName)

	log.Trace().Str("src", srcPath).Str("dst", dstPath).Msg("copying file")

	dstFile, err := os.OpenFile(dstPath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0600)
	if err != nil {
		return fmt.Errorf("unable to open destination file %q: %w", dstPath, err)
	}

	if _, err = io.Copy(dstFile, srcFile); err != nil {
		return fmt.Errorf("unable to copy from %q to %q: %w", srcPath, dstPath, err)
	}

	return nil
}
