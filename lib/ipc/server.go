package ipc

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"os/signal"

	"github.com/apex/log"
)

type ListenerConfig struct {
	AppName         string
	ReplaceIfExists bool
	PerProcess      bool
	Verbose         bool
}

func Serve(ctx context.Context, config *ListenerConfig) error {
	logger := log.FromContext(ctx)
	sockAddr, err := makeSocketAddr(config.AppName, config.PerProcess)
	ctx, stop := signal.NotifyContext(ctx, os.Interrupt, os.Kill)
	defer stop()

	// Create a unix domain socket for listening
	listener, err := net.Listen(sockAddr.ListenStreamArgs())
	if err != nil {
		logger.WithField("address", sockAddr).
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
	logger.WithField("address", sockAddr).Info("listening")

	// Wait for SIGINT or SIGKILL
	<-ctx.Done()
	return nil
}
