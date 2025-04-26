package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:     "thcon",
	Version: "0.17.1",
	Short:   "Switches apps between dark mode and light mode",
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		// PersistentPreRun executes after arg validation.
		// Silence usage here (instead of at the struct definition level),
		// to allow usage to be displayed with arg validation errors,
		// but not with other errors.
		cmd.SilenceUsage = true
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}
