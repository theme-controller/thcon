//go:build linux || dragonfly || freebsd || netbsd || openbsd

package apps_test

import (
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/require"
	"github.com/theme-controller/thcon/lib/config"
)

func TestGtk3Config(t *testing.T) {
	t.Run("can be omitted", func(t *testing.T) {
		var dst config.Config
		_, err := toml.Decode("", &dst)
		require.NoError(t, err)
		require.Nil(t, dst.Gtk3)
	})

	t.Run("can be disabled", func(t *testing.T) {
		var dst config.Config
		c := `
[gtk3]
disabled = true
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Gtk3)
		require.True(t, dst.Gtk3.Disabled)
	})

	t.Run("disabled by default when config present", func(t *testing.T) {
		var dst config.Config
		c := `
[gtk3]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Gtk3)
		require.False(t, dst.Gtk3.Disabled)
	})

	t.Run("includes dark mode and light mode themes", func(t *testing.T) {
		var dst config.Config
		c := `
[gtk3]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Gtk3)
		require.Equal(t, dst.Gtk3.Dark, "foo theme")
		require.Equal(t, dst.Gtk3.Light, "bar theme")
	})
}
