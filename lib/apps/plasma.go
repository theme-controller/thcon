package apps

import (
	"context"
	"os/exec"

	"github.com/apex/log"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/operation"
)

type Plasma struct {
	progress event.ProgressChannel
}

func NewPlasma(progress event.ProgressChannel) Switchable {
	return &Plasma{
		progress: progress,
	}
}

func (p *Plasma) Name() string {
	const name = "Plasma"
	return name
}

func (p *Plasma) Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error {
	logger := log.FromContext(ctx)

	var packageName string
	switch mode {
	case operation.DarkMode:
		packageName = "org.kde.breezedark.desktop"
	case operation.LightMode:
		packageName = "org.kde.breeze.desktop"
	}

	laft := exec.CommandContext(
		ctx,
		"lookandfeeltool",
		"--apply",
		packageName,
	)
	err := laft.Run()
	if err != nil {
		logger.WithError(err).Error("exec lookandfeeltool")
		return err
	}

	return nil
}
