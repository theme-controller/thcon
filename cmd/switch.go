package cmd

import (
	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib"
	"github.com/theme-controller/thcon/lib/apps"
	"github.com/theme-controller/thcon/lib/operation"
)

var lightCmd = &cobra.Command{
	Use:       "light [app...]",
	Short:     "Switches to apps to light mode",
	ValidArgs: apps.Argnames(),
	Args:      cobra.OnlyValidArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		return lib.Switch(cmd.Context(), operation.LightMode, args)
	},
}

var darkCmd = &cobra.Command{
	Use:       "dark [app...]",
	Short:     "Switches to apps to dark mode",
	ValidArgs: apps.Argnames(),
	Args:      cobra.OnlyValidArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		return lib.Switch(cmd.Context(), operation.DarkMode, args)
	},
}

func init() {
	lib.AddSwitchFlags(lightCmd)
	lib.AddSwitchFlags(darkCmd)
	lib.WithSwitchUsage(lightCmd)
	lib.WithSwitchUsage(darkCmd)

	rootCmd.AddCommand(lightCmd)
	rootCmd.AddCommand(darkCmd)
}
