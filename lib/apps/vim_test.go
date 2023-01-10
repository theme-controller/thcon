package apps_test

import (
	"testing"

	"github.com/BurntSushi/toml"
	"github.com/stretchr/testify/require"
	"github.com/theme-controller/thcon/lib/apps"
)

func TestVimConfig(t *testing.T) {
	t.Run("can be omitted", func(t *testing.T) {
		var dst apps.Config
		_, err := toml.Decode("", &dst)
		require.NoError(t, err)
		require.Nil(t, dst.Vim)
	})

	t.Run("can be disabled", func(t *testing.T) {
		var dst apps.Config
		c := `
[vim]
disabled = true
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Vim)
		require.True(t, dst.Vim.IsDisabled())
	})

	t.Run("disabled by default when config present", func(t *testing.T) {
		var dst apps.Config
		c := `
[vim]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Vim)
		require.False(t, dst.Vim.IsDisabled())
	})

	t.Run("includes dark mode and light mode themes", func(t *testing.T) {
		var dst apps.Config
		c := `
[vim]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Vim)
		require.Equal(t, dst.Vim.Dark, "foo theme")
		require.Equal(t, dst.Vim.Light, "bar theme")
	})
}

func TestNeovimConfig(t *testing.T) {
	t.Run("can be omitted", func(t *testing.T) {
		var dst apps.Config
		_, err := toml.Decode("", &dst)
		require.NoError(t, err)
		require.Nil(t, dst.Neovim)
	})

	t.Run("can be disabled", func(t *testing.T) {
		var dst apps.Config
		c := `
[nvim]
disabled = true
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Neovim)
		require.True(t, dst.Neovim.IsDisabled())
	})

	t.Run("disabled by default when config present", func(t *testing.T) {
		var dst apps.Config
		c := `
[nvim]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Neovim)
		require.False(t, dst.Neovim.IsDisabled())
	})

	t.Run("includes dark mode and light mode themes", func(t *testing.T) {
		var dst apps.Config
		c := `
[nvim]
dark = "foo theme"
light = "bar theme"
`
		_, err := toml.Decode(c, &dst)
		require.NoError(t, err)
		require.NotNil(t, dst.Neovim)
		require.Equal(t, dst.Neovim.Dark, "foo theme")
		require.Equal(t, dst.Neovim.Light, "bar theme")
	})
}
