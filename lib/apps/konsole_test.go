//go:build linux || dragonfly || freebsd || netbsd || openbsd

package apps_test

import (
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/require"
	"github.com/theme-controller/thcon/lib/config"
)

func TestKonsoleConfig(t *testing.T) {
	t.Run("can be omitted", func(t *testing.T) {
		var dst config.Config
		_, err := toml.Decode("", &dst)
		require.NoError(t, err)
		require.Nil(t, dst.Konsole)
	})

	t.Run("can be disabled", func(t *testing.T) {
		var dst config.Config
		c := `
[konsole]
disabled = true
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Konsole)
		require.True(t, dst.Konsole.Disabled)
	})

	t.Run("disabled by default when config present", func(t *testing.T) {
		var dst config.Config
		c := `
[konsole]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Konsole)
		require.False(t, dst.Konsole.Disabled)
	})

	t.Run("includes dark mode and light mode themes", func(t *testing.T) {
		var dst config.Config
		c := `
[konsole]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Konsole)
		require.Equal(t, dst.Konsole.Dark, "foo theme")
		require.Equal(t, dst.Konsole.Light, "bar theme")
	})
}
