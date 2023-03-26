package apps

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"syscall"

	"github.com/mitchellh/go-homedir"
	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/ipc"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
)

type VimConfigSlice struct {
	Vim *vimConfig `toml:"vim"`
}
type vimConfig struct {
	health.Disabled
	Dark  string `toml:"dark" validate:"expfile,required_with=Light"`
	Light string `toml:"light" validate:"expfile,required_with=Dark"`
}

type NeovimConfigSlice struct {
	Neovim *neovimConfig `toml:"nvim"`
}
type neovimConfig struct {
	health.Disabled
	Dark  string `toml:"dark" validate:"expfile,required_with=Light,sameExtAs=Light"`
	Light string `toml:"light" validate:"expfile,required_with=Dark,sameExtAs=Dark"`
}

type anyVim struct {
	flavor string
}

var _ Switchable = (*anyVim)(nil)

func NewVim() Switchable {
	return &anyVim{
		flavor: "vim",
	}
}

func NewNeovim() Switchable {
	return &anyVim{
		flavor: "neovim",
	}
}

func (v *anyVim) Name() string {
	return v.flavor
}

func (v *anyVim) Argname() string {
	return v.flavor
}

func (v *anyVim) sockbase() string {
	if v.flavor == "neovim" {
		return "nvim"
	}
	return "vim"
}

func (v *anyVim) getSymlinkTarget(rcFile string) (string, error) {
	thcon_dir, err := util.EnsureThconStateDir()
	if err != nil {
		return "", err
	}

	var extension string = filepath.Ext(rcFile)
	// Lua syntax must be in a .lua file extension, but any other extension
	// can contain VimL. Strip non-lua extensions, to produce symlinks called
	// either 'neovim.lua' (for lua configs) or 'neovimrc'/'vimrc' (for VimL).
	if extension != ".lua" {
		extension = "rc"
	}
	return filepath.Join(thcon_dir, v.flavor+extension), nil
}

func (v *anyVim) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	if v.flavor == "neovim" {
		cfg := config.Neovim
		return health.RequiresConfig(ctx, cfg)
	}

	cfg := config.Vim
	return health.RequiresConfig(ctx, cfg)
}

func (v *anyVim) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	var rc_file string
	if v.flavor == "neovim" {
		var themeConfig *neovimConfig = config.Neovim
		if themeConfig == nil {
			themeConfig = &neovimConfig{
				Dark:  "~/.config/nvim/lua/dark.thcon.lua",
				Light: "~/.config/nvim/lua/light.thcon.lua",
			}
		}
		if mode == operation.DarkMode {
			rc_file = themeConfig.Dark
		} else {
			rc_file = themeConfig.Light
		}
	} else {
		var themeConfig *vimConfig = config.Vim
		if themeConfig == nil {
			themeConfig = &vimConfig{
				Dark:  "~/dark.thcon.vimrc",
				Light: "~/light.thcon.vimrc",
			}
		}
		if mode == operation.DarkMode {
			rc_file = themeConfig.Dark
		} else {
			rc_file = themeConfig.Light
		}
	}

	// 1) Symlink a thcon state file to point to the desired config.
	rc_file, err := homedir.Expand(rc_file)
	if err != nil {
		return err
	}

	symlink_target, err := v.getSymlinkTarget(rc_file)
	if err != nil {
		return err
	}

	exists, err := util.SymlinkExists(symlink_target)
	if err != nil {
		return nil
	}
	if exists {
		if err := os.Remove(symlink_target); err != nil {
			return err
		}
	}
	if err := os.Symlink(rc_file, symlink_target); err != nil {
		return err
	}

	// 2) Notify all vim or nvim processes over IPC (via `thcon listen`).
	type IpcMessage struct {
		RcFile string `json:"rc_file"`
	}
	msg, err := json.Marshal(IpcMessage{rc_file})
	if err != nil {
		return err
	}
	socks, err := ipc.ListSockets(v.sockbase(), true)
	if err != nil {
		return err
	}

	var writeFailure bool
	for _, sock := range socks {
		payload := &ipc.Outbound{
			Socket:  sock,
			Message: msg,
		}
		if err := ipc.Send(ctx, payload); err != nil {
			writeFailure = true
			if errors.Is(err, syscall.ECONNREFUSED) {
				log.Warn().
					Stringer("sock", sock).
					Msg("cleaning up abandoned socket")
				_ = os.Remove(sock.Path())
			} else {
				log.Error().
					Err(err).
					Msg("apply settings")
			}
		}
	}

	if writeFailure {
		return errors.New("Failed to apply settings")
	}
	return nil
}
