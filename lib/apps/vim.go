package apps

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"syscall"

	"github.com/apex/log"
	"github.com/mitchellh/go-homedir"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/ipc"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
)

type anyVimConfig struct {
	Disabled bool   `toml:"disabled"`
	Dark     string `toml:"dark"`
	Light    string `toml:"light"`
}

type AllVimConfig struct {
	Vim    *anyVimConfig `toml:"vim"`
	Neovim *anyVimConfig `toml:"nvim"`
}

type anyVim struct {
	progress event.ProgressChannel
	flavor   string
}

var _ Switchable = (*anyVim)(nil)

func NewVim(progress event.ProgressChannel) Switchable {
	return &anyVim{
		flavor:   "vim",
		progress: progress,
	}
}

func NewNeovim(progress event.ProgressChannel) Switchable {
	return &anyVim{
		flavor:   "neovim",
		progress: progress,
	}
}

func (v *anyVim) Name() string {
	return v.flavor
}

func (v *anyVim) sockbase() string {
	if v.flavor == "neovim" {
		return "nvim"
	}
	return "vim"
}

func (v *anyVim) Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error {
	logger := log.FromContext(ctx)

	var rc_file string
	switch mode {
	case operation.DarkMode:
		rc_file = "~/.config/nvim/lua/sjbarag/dark.lua"
	case operation.LightMode:
		rc_file = "~/.config/nvim/lua/sjbarag/light.lua"
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
	if len(socks) > 1 {
		v.progress <- event.AddSubsteps(v.flavor, len(socks))
	}

	var writeFailure bool
	for idx, sock := range socks {
		if idx > 1 {
			v.progress <- event.StepStarted(v.flavor)
		}

		payload := &ipc.Outbound{
			Socket:  sock,
			Message: msg,
		}
		if err := ipc.Send(ctx, payload); err != nil {
			writeFailure = true
			v.progress <- event.StepFailed(v.flavor, err)
			if errors.Is(err, syscall.ECONNREFUSED) {
				logger.WithField("sock", sock).Warn("cleaning up abandoned socket")
				_ = os.Remove(sock.Path())
			} else {
				logger.
					WithError(err).
					Error("apply settings")
			}
		}
		if idx > 1 {
			v.progress <- event.StepCompleted(v.flavor)
		}
	}

	if writeFailure {
		return errors.New("Failed to apply settings")
	}
	return nil
}
