package cmd

import (
	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib"
	"github.com/theme-controller/thcon/lib/operation"
)

var lightCmd = &cobra.Command{
	Use:   "light",
	Short: "Switches to apps to light mode",
	RunE: func(cmd *cobra.Command, args []string) error {
		return lib.Switch(cmd.Context(), operation.LightMode)
	},
}

var darkCmd = &cobra.Command{
	Use:   "dark",
	Short: "Switches to apps to dark mode",
	RunE: func(cmd *cobra.Command, args []string) error {
		return lib.Switch(cmd.Context(), operation.DarkMode)
	},
}

func init() {
	lib.AddSwitchFlags(lightCmd)
	lib.AddSwitchFlags(darkCmd)

	rootCmd.AddCommand(lightCmd)
	rootCmd.AddCommand(darkCmd)
}
