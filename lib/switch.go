package lib

import (
	"context"
	"errors"
	"fmt"
	"os"
	"runtime"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib/apps"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
	"golang.org/x/sync/errgroup"
)

const (
	verboseFlag = "verbose"
)

var verbosity int

func Switch(ctx context.Context, mode operation.Operation) error {
	if verbosity < 0 {
		verbosity = 0
	}
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})
	switch verbosity {
	case 0:
		zerolog.SetGlobalLevel(zerolog.WarnLevel)
	case 1:
		zerolog.SetGlobalLevel(zerolog.InfoLevel)
	case 2:
		zerolog.TimeFieldFormat = zerolog.TimeFormatUnixMs
		zerolog.SetGlobalLevel(zerolog.DebugLevel)
	default:
		zerolog.TimeFieldFormat = zerolog.TimeFormatUnixMicro
		zerolog.SetGlobalLevel(zerolog.TraceLevel)
	}

	switch mode {
	case operation.DarkMode:
		fmt.Println("Switching to dark mode")
	case operation.LightMode:
		fmt.Println("Switching to light mode")
	default:
		return fmt.Errorf("Unexpected mode '%+v'", mode)
	}

	// TODO: make apps dynamic
	allApps := apps.All()

	configPath, err := apps.ConfigFilePath()
	if err != nil {
		return err
	}
	config, err := apps.ParseConfig(ctx, configPath)
	if err != nil {
		log.Error().Err(err).Msg("Unable to parse thcon.toml")
		return err
	}
	log.Debug().Stringer("config", config).Msg("found config")

	// Validate configs
	var toSwitch []apps.Switchable
	validator := goValidator.New()
	for _, app := range allApps {
		err := app.ValidateConfig(ctx, validator, config)
		if err == nil {
			toSwitch = append(toSwitch, app)
			continue
		}

		var valErrs goValidator.ValidationErrors
		if errors.As(err, &valErrs) {
			for _, err := range valErrs {
				log.Warn().
					Str("app", app.Name()).
					Err(err).
					Msg("validate config")
			}
		} else if errors.Is(err, apps.ErrNeedsConfig) {
			log.Info().
				Str("app", app.Name()).
				Msg("app disabled: needs configuration")
		} else {
			log.Error().
				Str("app", app.Name()).
				Err(err).
				Msg("unexpected validation error")
		}
	}

	var numErrors int

	// Switch all as parallelibly as possible
	g := errgroup.Group{}
	g.SetLimit(runtime.GOMAXPROCS(0))

	for _, app := range toSwitch {
		app := app
		name := app.Name()
		appLog := log.With().
			Str("app", name).
			Logger()
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
		return errors.New("One app failed to switch themes")
	default:
		var count string = "Some"
		if numErrors == len(toSwitch) {
			count = "All"
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
