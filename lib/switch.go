package lib

import (
	"fmt"

	"github.com/spf13/cobra"
)

type Operation int

const (
	DarkMode Operation = iota
	LightMode

	verboseFlag  = "verbose"
	progressFlag = "progress"
)

var verbosity int
var showProgress bool

func Switch(mode Operation) error {
	switch mode {
	case DarkMode:
		fmt.Println("Switching to dark mode")
	case LightMode:
		fmt.Println("Switching to light mode")
	default:
		return fmt.Errorf("Unexpected mode '%+v'", mode)
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
