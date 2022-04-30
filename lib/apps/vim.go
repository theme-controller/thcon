package apps

import (
	"context"
	"errors"

	"github.com/apex/log"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/ipc"
	"github.com/theme-controller/thcon/lib/operation"
)

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

func (v *anyVim) Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error {
	logger := log.FromContext(ctx)
	socks, err := ipc.ListSockets(v.flavor, true)
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
			Message: []byte(`{ foo: "hello vim" }`),
		}
		if err := ipc.Send(ctx, payload); err != nil {
			writeFailure = true
			v.progress <- event.StepFailed(v.flavor, err)
			logger.
				WithError(err).
				Error("apply settings")
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
