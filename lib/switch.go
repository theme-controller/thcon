package lib

import (
	"context"
	"errors"
	"fmt"
	"os"
	"os/signal"
	"strings"
	"time"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib/apps"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
	"golang.org/x/sync/errgroup"
)

const (
	verboseFlag = "verbose"
)

var verbosity int

func Switch(ctx context.Context, mode operation.Operation, args []string) error {
	ctx, stop := signal.NotifyContext(ctx, os.Interrupt, os.Kill)
	defer stop()

	if verbosity < 0 {
		verbosity = 0
	}

	writer := zerolog.ConsoleWriter{Out: os.Stderr}
	switch verbosity {
	case 0:
		zerolog.SetGlobalLevel(zerolog.WarnLevel)
	case 1:
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
	case 2:
		zerolog.TimeFieldFormat = time.RFC3339Nano
		writer.TimeFormat = "15:04:05.000"
		zerolog.SetGlobalLevel(zerolog.DebugLevel)
	default:
		zerolog.TimeFieldFormat = time.RFC3339Nano
		writer.TimeFormat = "15:04:05.000000"
		zerolog.SetGlobalLevel(zerolog.TraceLevel)
	}
	log.Logger = log.Output(writer)

	switch mode {
	case operation.DarkMode:
		fmt.Println("Switching to dark mode")
	case operation.LightMode:
		fmt.Println("Switching to light mode")
	default:
		return fmt.Errorf("unexpected mode '%+v'", mode)
	}

	// TODO: maybe put this in context and pass it through to the health.___ functions?
	// It'd be nice to still get validation errors for a disabled app, but without having to implement IsDisabled()
	// on every config struct or have to do that casting here.
	// Or perhaps the casting to *Disabled belongs somewhere that isn't lib/health/health.go?
	userRequestedApps := len(args) > 0

	chosenApps := []apps.Switchable{}
	if userRequestedApps {
		apps := apps.Map()
		for _, arg := range args {
			if app, found := apps[arg]; found {
				chosenApps = append(chosenApps, app)
			}
		}
	} else {
		chosenApps = apps.All
	}

	configPath, err := apps.ConfigFilePath()
	if err != nil {
		return err
	}
	config, err := apps.ParseConfig(ctx, configPath)
	if err != nil {
		log.Error().Err(err).Msg("Unable to parse thcon.toml")
		return err
	}

	// TODO: figure out how to make extensions show up on the command line without
	// having to parse the entire config just for --help and completion. It might
	// not be possible.
	chosenApps = append(chosenApps, apps.MakeExtensions(ctx, config)...)

	// Validate configs
	toSwitch := []apps.Switchable{}
	for _, app := range chosenApps {
		appLog := log.With().Str("app", app.Name()).Logger()
		status, err := app.ValidateConfig(ctx, config)

		switch status {
		case health.StatusDisabled:
			appLog.Info().Str("reason", "disabled").Msg("skipping")
		case health.StatusNotInstalled:
			appLog.Info().Str("reason", "not installed").Msg("skipping")
		case health.StatusMissingConfig:
			appLog.Info().Str("reason", "requires config").Msg("skipping")
		case health.StatusNotOk:
			if err == nil {
				appLog.Error().Err(nil).Str("reason", "unknown").Msg("skipping")
				continue
			}

			if verrs, ok := err.(goValidator.ValidationErrors); ok {
				appLog.Error().
					Str("reason", "validation failed").
					Errs("validation errors", health.ValidationErrorsToErrorSlice(verrs)).
					Msg("skipping")
				continue
			}

			appLog.Error().Err(err).Str("reason", "unexpected error").Msg("skipping")
		case health.StatusOk:
			toSwitch = append(toSwitch, app)
		}
	}

	var numErrors int

	// Switch all as parallelibly as possible
	ctx, cancel := context.WithTimeout(ctx, 3*time.Second)
	g, ctx := errgroup.WithContext(ctx)
	defer cancel()

	for _, app := range toSwitch {
		app := app
		name := app.Name()
		appLog := log.With().
			Str("app", name).
			Logger()
		ctx := ctx
		appCtx := appLog.WithContext(ctx)

		appLog.Trace().Msg("queueing")
		g.Go(func() error {
			var err error
			appLog.Debug().Msg(mode.Verb())
			dur := appLog.Hook(util.RecordDuration())

			err = app.Switch(appCtx, mode, config)
			if err != nil {
				appLog.Error().Err(err).Msg("failed")
				numErrors++

				// Always return nil, to allow other apps to switch
				// even if the current one encounters an error.
				return nil
			}

			dur.Trace().Msg("done")

			// Always return nil, to allow other apps to switch
			// even if the current one encounters an error.
			return nil
		})
	}

	// Ignore the returned error, since it's always nil.
	_ = g.Wait()

	switch numErrors {
	case 0:
		return nil
	case 1:
		return errors.New("one app failed to switch themes")
	default:
		var count string = "some"
		if numErrors == len(toSwitch) {
			count = "all"
		}
		return fmt.Errorf("%s apps failed to switch themes", count)
	}
}

func AddSwitchFlags(cmd *cobra.Command) {
	cmd.Flags().CountVarP(
		&verbosity,
		verboseFlag,
		"v",
		"enable verbose logging (add multiple times for higher verbosity)",
	)
}

func WithSwitchUsage(cmd *cobra.Command) {
	cobra.AddTemplateFunc("commaJoin", func(args []string) string {
		return strings.Join(args, ", ")
	})

	// Copied from https://github.com/spf13/cobra/blob/b43be995ebb4bee335a787bd44498b91aef7619c/command.go#L539-L568
	// then modified.
	cmd.SetUsageTemplate(`Usage:{{if .Runnable}}
  {{.UseLine}}{{end}}{{if .HasAvailableSubCommands}}
  {{.CommandPath}} [command]{{end}}{{if gt (len .Aliases) 0}}

Aliases:
  {{.NameAndAliases}}{{end}}{{if .HasExample}}

Examples:
{{.Example}}{{end}}{{if .HasAvailableSubCommands}}{{$cmds := .Commands}}{{if eq (len .Groups) 0}}

Available Commands:{{range $cmds}}{{if (or .IsAvailableCommand (eq .Name "help"))}}
  {{rpad .Name .NamePadding }} {{.Short}}{{end}}{{end}}{{else}}{{range $group := .Groups}}

{{.Title}}{{range $cmds}}{{if (and (eq .GroupID $group.ID) (or .IsAvailableCommand (eq .Name "help")))}}
  {{rpad .Name .NamePadding }} {{.Short}}{{end}}{{end}}{{end}}{{if not .AllChildCommandsHaveGroup}}

Additional Commands:{{range $cmds}}{{if (and (eq .GroupID "") (or .IsAvailableCommand (eq .Name "help")))}}
  {{rpad .Name .NamePadding }} {{.Short}}{{end}}{{end}}{{end}}{{end}}{{end}}

Apps:
  {{commaJoin .ValidArgs | trimTrailingWhitespaces}}{{if .HasAvailableLocalFlags}}

Flags:
{{.LocalFlags.FlagUsages | trimTrailingWhitespaces}}{{end}}{{if .HasAvailableInheritedFlags}}

Global Flags:
{{.InheritedFlags.FlagUsages | trimTrailingWhitespaces}}{{end}}{{if .HasHelpSubCommands}}

Additional help topics:{{range .Commands}}{{if .IsAdditionalHelpTopicCommand}}
  {{rpad .CommandPath .CommandPathPadding}} {{.Short}}{{end}}{{end}}{{end}}{{if .HasAvailableSubCommands}}

Use "{{.CommandPath}} [command] --help" for more information about a command.{{end}}
`)
}
