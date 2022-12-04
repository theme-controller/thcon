package config

import (
	"context"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestConfigPath(t *testing.T) {
	res, err := ConfigFilePath()
	require.NoError(t, err)

	switch runtime.GOOS {
	case "linux":
	case "darwin":
		home, err := os.UserHomeDir()
		require.NoError(t, err)
		require.Equal(
			t,
			filepath.Join(home, ".config", "thcon", "thcon.toml"),
			res,
		)
	case "window":
		require.Fail(t, "Windows support isn't ready yet")
	default:
		require.Failf(t, "Unsupported platform '%s'", runtime.GOOS)
	}
}

func TestParse(t *testing.T) {
	t.Run("Ignores unused sections", func(t *testing.T) {
		f, err := ioutil.TempFile("", "thcon_config_unused_*.toml")
		require.NoError(t, err)
		defer os.Remove(f.Name())

		_, err = f.WriteString(
			strings.TrimSpace(`
				[foo]
				bar = "baz"
			`),
		)
		require.NoError(t, err)

		_, err = Parse(context.Background(), f.Name())
		require.NoError(t, err)
	})
}
