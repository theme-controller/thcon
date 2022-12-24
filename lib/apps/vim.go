package apps

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"syscall"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/mitchellh/go-homedir"
	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/ipc"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
)

type VimConfigSlice struct {
	Vim *vimConfig `toml:"vim"`
}
type vimConfig struct {
	Disabled bool   `toml:"disabled"`
	Dark     string `toml:"dark" validate:"file,required_with=Light"`
	Light    string `toml:"light" validate:"file,required_with=Dark"`
}

type NeovimConfigSlice struct {
	Neovim *neovimConfig `toml:"nvim"`
}
type neovimConfig struct {
	Disabled bool   `toml:"disabled"`
	Dark     string `toml:"dark" validate:"required_with=Light"`
	Light    string `toml:"light" validate:"required_with=Dark"`
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

func (v *anyVim) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) error {
	if v.flavor == "neovim" {
		cfg := config.Neovim
		if cfg == nil {
			return ErrNeedsConfig
		}
		return validator.StructCtx(ctx, cfg)
	}

	cfg := config.Vim
	if cfg == nil {
		return ErrNeedsConfig
	}
	return validator.StructCtx(ctx, cfg)
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

	rc_file, err := homedir.Expand(rc_file)
	if err != nil {
		return err
	}

	thcon_dir, err := util.EnsureThconStateDir()
	if err != nil {
		return err
	}

	var extension string = filepath.Ext(rc_file)
	if extension != ".lua" {
		extension = ""
	}
	symlink_target := filepath.Join(thcon_dir, v.flavor+extension)
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
