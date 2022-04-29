package ipc

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"os/signal"
	"os/user"
	"path/filepath"

	"github.com/apex/log"
)

type ListenerConfig struct {
	AppName         string
	ReplaceIfExists bool
	PerProcess      bool
	Verbose         bool
}

func ensureUserSocketDir() (string, error) {
	currUser, err := user.Current()
	if err != nil {
		return "", fmt.Errorf("Unable to create socket directory for current user: %+v", err)
	}
	dirname := filepath.Join(os.TempDir(), "thcon-"+currUser.Uid)
	err = os.MkdirAll(dirname, 0600)
	if err != nil {
		return "", fmt.Errorf("Unable to create socket directory for current user: %+v", err)
	}

	return dirname, nil
}

func Serve(ctx context.Context, config *ListenerConfig) error {
	logger := log.FromContext(ctx)
	dir, err := ensureUserSocketDir()
	if err != nil {
		return err
	}
	var filename string
	if config.PerProcess {
		filename = fmt.Sprintf("%s-%d.sock", config.AppName, os.Getpid())
	} else {
		filename = config.AppName + ".sock"
	}
	sockName := filepath.Join(dir, filename)

	ctx, stop := signal.NotifyContext(ctx, os.Interrupt, os.Kill)
	defer stop()

	// Create a unix domain socket for listening
	listener, err := net.Listen("unix", sockName)
	if err != nil {
		logger.WithField("address", sockName).
			WithError(err).
			Error("Unable to listen on unix socket")
		return err
	}
	defer listener.Close()

	// Serve HTTP on that socket
	go http.Serve(
		listener,
		http.HandlerFunc(
			func(w http.ResponseWriter, r *http.Request) {
				bodyBytes, err := io.ReadAll(r.Body)
				if err != nil {
					logger.WithError(err).Error("Unable to read request")
					w.WriteHeader(http.StatusInternalServerError)
					return
				}
				fmt.Println(string(bodyBytes))
			},
		),
	)
	logger.WithField("address", sockName).Info("listening")

	// Wait for SIGINT or SIGKILL
	<- ctx.Done()
	return nil
}
