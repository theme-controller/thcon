package lib

import (
	"context"
	"errors"
	"fmt"
	"runtime"
	"sync"

	"github.com/apex/log"
	"github.com/apex/log/handlers/cli"
	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib/apps"
	"github.com/theme-controller/thcon/lib/config"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/operation"
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
	switch verbosity {
	case 0:
		log.SetLevel(log.WarnLevel)
	case 1:
		log.SetLevel(log.InfoLevel)
	default:
		log.SetLevel(log.DebugLevel)
	}
	log.SetHandler(cli.Default)

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

	config, err := config.Parse(ctx)
	if err != nil {
		log.WithError(err).Error("Unable to parse thcon.toml")
		return err
	}
	log.WithField("config", config).Info("found config")

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
				log.
					WithField("type", "add steps").
					WithField("count", evt.SubstepCount).
					Debug("progress")
				totalSteps += evt.SubstepCount
				inProgress += evt.SubstepCount
			case event.KStepStarted:
				log.
					WithField("type", "start step").
					Debug("progress")
				inProgress++
			case event.KStepCompleted:
				log.
					WithField("type", "finish step").
					Debug("progress")
				inProgress--
				complete++
			case event.KStepFailed:
				log.
					WithField("type", "fail step").
					Debug("progress")
				inProgress--
				failed++
			}

			log.WithFields(log.Fields{
				"total":      totalSteps,
				"complete":   complete,
				"failed":     failed,
				"inProgress": inProgress,
			}).Debug("progress")
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
		appLog := log.WithField("app", name)
		appLog.Debug("requesting lock")
		appCtx := log.NewContext(ctx, appLog)
		sem <- 1
		progressChan <- event.StepStarted(name)

		go func() {
			var err error
			defer wg.Done()
			defer func() { <-sem }()
			defer appLog.Trace(mode.Verb()).Stop(&err)

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
