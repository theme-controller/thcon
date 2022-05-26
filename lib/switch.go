package lib

import (
	"context"
	"errors"
	"fmt"
	"os"
	"runtime"
	"sync"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib/apps"
	"github.com/theme-controller/thcon/lib/config"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/operation"
	"github.com/theme-controller/thcon/lib/util"
)

const (
	verboseFlag  = "verbose"
	progressFlag = "progress"
)

var verbosity int
var showProgress bool

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
	var progressChanBuf int = 2
	maxProcs := runtime.GOMAXPROCS(0)
	if maxProcs > progressChanBuf {
		progressChanBuf = maxProcs - 1
	}

	progressChan := make(chan *event.ProgressEvent, progressChanBuf)
	toSwitch := apps.All(progressChan)

	configPath, err := config.ConfigFilePath()
	if err != nil {
		return err
	}
	config, err := config.Parse(ctx, configPath)
	if err != nil {
		log.Error().Err(err).Msg("Unable to parse thcon.toml")
		return err
	}
	log.Info().Str("config", config.String()).Msg("found config")

	// Render progress events
	progressDone := make(chan bool)
	go func() {
		var totalSteps int = 2
		var complete int
		var inProgress int
		var failed int

		for evt := range progressChan {
			switch evt.Kind {
			case event.KAddSubsteps:
				totalSteps += evt.SubstepCount
				inProgress += evt.SubstepCount
			case event.KStepStarted:
				inProgress++
			case event.KStepCompleted:
				inProgress--
				complete++
			case event.KStepFailed:
				inProgress--
				failed++
			}

			log.Trace().
				Err(evt.Err).
				Int("total", totalSteps).
				Int("complete", complete).
				Int("failed", failed).
				Int("inProgress", inProgress).
				Msg(evt.Kind.ToString())
		}
		progressDone <- true
	}()

	var numErrors int

	// Switch all as parallelibly as possible
	sem := make(chan int, maxProcs)
	wg := sync.WaitGroup{}
	for _, app := range toSwitch {
		wg.Add(1)

		app := app
		name := app.Name()
		// TODO: keep switching to zerolog
		appLog := log.With().
			Str("app", name).
			Logger()
		appLog.Debug().Msg("requesting lock")
		appCtx := appLog.WithContext(ctx)
		sem <- 1
		progressChan <- event.StepStarted(name)

		go func() {
			var err error
			defer wg.Done()
			defer func() { <-sem }()
			appLog.Debug().Msg(mode.Verb())
			dur := appLog.Hook(util.RecordDuration())
			defer dur.Trace().Msg("done")

			err = app.Switch(appCtx, mode, nil)
			if err != nil {
				numErrors++
				progressChan <- event.StepFailed(name, err)
			} else {
				progressChan <- event.StepCompleted(name)
			}
		}()
	}

	// Wait for all apps to finish switching
	wg.Wait()
	// Flush remaining progress events
	close(progressChan)
	<-progressDone

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

	cmd.Flags().BoolVar(
		&showProgress,
		progressFlag,
		false,
		"show progress while switching apps",
	)
}
