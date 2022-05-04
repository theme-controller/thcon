package cmd

import (
	"github.com/apex/log"
	"github.com/apex/log/handlers/text"
	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib/ipc"
)

var perProcess bool
var shouldReplace bool
var verboseListen bool

var listenCmd = &cobra.Command{
	Use:           "listen",
	Short:         "Listens for messages on a socket, printing them to stdout",
	Args:          cobra.ExactArgs(1),
	SilenceErrors: false,
	SilenceUsage:  false,
	RunE: func(cmd *cobra.Command, args []string) error {
		appName := args[0]
		if appName == "help" {
			return cmd.Help()
		}

		ctx := cmd.Context()
		if verboseListen {
			log.SetLevel(log.DebugLevel)
		} else {
			log.SetLevel(log.WarnLevel)
		}
		log.SetHandler(text.Default)
		log.NewContext(ctx, log.Log)

		return ipc.Serve(ctx, &ipc.ListenerConfig{
			AppName:         args[0],
			PerProcess:      perProcess,
			ReplaceIfExists: shouldReplace,
			Verbose:         verboseListen,
		})
	},
}

func init() {
	listenCmd.Flags().BoolVar(
		&shouldReplace, "replace", true,
		"Exits if a socket already exists, instead of replacing it",
	)
	listenCmd.Flags().BoolVar(
		&perProcess, "per-process", false,
		"Creates a connection unique to this process",
	)
	listenCmd.Flags().BoolVarP(
		&verboseListen, "verbose", "v", false,
		"Enables verbose output",
	)
	rootCmd.AddCommand(listenCmd)
}
