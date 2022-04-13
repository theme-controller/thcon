package lib

import (
	"context"
	"fmt"
	"log"
	"runtime"
	"sync"

	"github.com/spf13/cobra"
	"github.com/theme-controller/thcon/lib/apps"
	"github.com/theme-controller/thcon/lib/event"
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

	// TODO: make apps dynamic
	var progressChanBuf int = 2
	maxProcs := runtime.GOMAXPROCS(0)
	if maxProcs > progressChanBuf {
		progressChanBuf = maxProcs - 1
	}

	progressChan := make(chan *event.ProgressEvent, progressChanBuf)
	toSwitch := []apps.Switchable{
		apps.NewGtk(progressChan),
		apps.NewGnomeTerminal(progressChan),
	}

	// Render progress events
	progressDone := make(chan bool)
	go func() {
		for event := range progressChan {
			log.Printf("[PRG] received progress event: %#v", event)
		}
		progressDone <- true
	}()

	// Switch all as parallelibly as possible
	sem := make(chan int, maxProcs)
	wg := sync.WaitGroup{}
	for _, app := range toSwitch {
		wg.Add(1)

		app := app
		name := app.Name()
		log.Printf("[DBG] requesting lock for %s", name)
		sem <- 1
		progressChan <- event.StepStarted(name)

		go func() {
			defer wg.Done()
			defer func() { <-sem }()

			log.Printf("[DBG] starting %s", name)
			err := app.Switch(context.Background(), mode, nil)
			if err != nil {
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
