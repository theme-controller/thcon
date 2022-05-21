package apps_test

import (
	"fmt"
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"
	"github.com/theme-controller/thcon/lib/config"
)

func TestGnomeTerminalConfig(t *testing.T) {
	const (
		dark_theme_id  = "dddddddd-0000-4000-0000-000000000000"
		light_theme_id = "11111111-0000-4000-0000-000000000000"
	)

	t.Run("can be omitted", func(t *testing.T) {
		var dst config.Config
		_, err := toml.Decode("", &dst)
		require.NoError(t, err)
		require.Nil(t, dst.GnomeTerminal)
	})

	t.Run("can be disabled", func(t *testing.T) {
		var dst config.Config
		c := `
[gnome-terminal]
disabled = true
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.GnomeTerminal)
		require.True(t, dst.GnomeTerminal.Disabled)
	})

	t.Run("disabled by default when config present", func(t *testing.T) {
		var dst config.Config
		c := fmt.Sprintf(`
[gnome-terminal]
dark = "%s"
light = "%s"
`, dark_theme_id, light_theme_id)

		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.GnomeTerminal)
		require.False(t, dst.GnomeTerminal.Disabled)
	})

	t.Run("includes dark mode and light mode themes", func(t *testing.T) {
		var dst config.Config
		c := fmt.Sprintf(`
[gnome-terminal]
dark = "%s"
light = "%s"
`, dark_theme_id, light_theme_id)

		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.GnomeTerminal)

		require.NoError(t, err)
		require.Equal(t, dst.GnomeTerminal.Dark, uuid.MustParse(dark_theme_id))
		require.Equal(t, dst.GnomeTerminal.Light, uuid.MustParse(light_theme_id))
	})
}
