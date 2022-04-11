package lib

import (
	"context"
	"fmt"

	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib/apps"
	"github.com/theme-controller/thcon/lib/operation"
)

const (
	verboseFlag  = "verbose"
	progressFlag = "progress"
)

var verbosity int
var showProgress bool

func Switch(mode operation.Operation) error {
	switch mode {
	case operation.DarkMode:
		fmt.Println("Switching to dark mode")
	case operation.LightMode:
		fmt.Println("Switching to light mode")
	default:
		return fmt.Errorf("Unexpected mode '%+v'", mode)
	}

	gtk := apps.Gtk{}
	err := gtk.Switch(context.Background(), mode, nil)
	if err != nil {
		return err
	}

	gnomeTerm := apps.GnomeTerminal{}
	err = gnomeTerm.Switch(context.Background(), mode, nil)
	if err != nil {
		return err
	}
	return nil
}

func AddSwitchFlags(cmd *cobra.Command) {
	cmd.Flags().CountVarP(
		&verbosity,
		verboseFlag,
		"v",
		"enable verbose logging (add multiple times for higher verbosity)",
	)

	cmd.Flags().BoolVar(
		&showProgress,
		progressFlag,
		false,
		"show progress while switching apps",
	)
}
