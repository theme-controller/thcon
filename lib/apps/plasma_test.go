package apps_test

import (
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/require"
	"github.com/theme-controller/thcon/lib/config"
)

func TestPlasmaConfig(t *testing.T) {
	t.Run("can be omitted", func(t *testing.T) {
		var dst config.Config
		_, err := toml.Decode("", &dst)
		require.NoError(t, err)
		require.Nil(t, dst.Plasma)
	})

	t.Run("can be disabled", func(t *testing.T) {
		var dst config.Config
		c := `
[plasma]
disabled = true
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Plasma)
		require.True(t, dst.Plasma.Disabled)
	})

	t.Run("disabled by default when config present", func(t *testing.T) {
		var dst config.Config
		c := `
[plasma]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Plasma)
		require.False(t, dst.Plasma.Disabled)
	})

	t.Run("includes dark mode and light mode themes", func(t *testing.T) {
		var dst config.Config
		c := `
[plasma]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Plasma)
		require.Equal(t, dst.Plasma.Dark, "foo theme")
		require.Equal(t, dst.Plasma.Light, "bar theme")
	})
}
