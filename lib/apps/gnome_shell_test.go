package apps_test

import (
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/require"
	"github.com/theme-controller/thcon/lib/config"
)

func TestGnomeShellConfig(t *testing.T) {
	t.Run("can be omitted", func(t *testing.T) {
		var dst config.Config
		_, err := toml.Decode("", &dst)
		require.NoError(t, err)
		require.Nil(t, dst.GnomeShell)
	})

	t.Run("can be disabled", func(t *testing.T) {
		var dst config.Config
		c := `
[gnome-shell]
disabled = true
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.GnomeShell)
		require.True(t, dst.GnomeShell.Disabled)
	})

	t.Run("disabled by default when config present", func(t *testing.T) {
		var dst config.Config
		c := `
[gnome-shell]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.GnomeShell)
		require.False(t, dst.GnomeShell.Disabled)
	})

	t.Run("includes dark mode and light mode themes", func(t *testing.T) {
		var dst config.Config
		c := `
[gnome-shell]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.GnomeShell)
		require.Equal(t, dst.GnomeShell.Dark, "foo theme")
		require.Equal(t, dst.GnomeShell.Light, "bar theme")
	})
}
