package ipc

import (
	"context"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"os/signal"

	"github.com/rs/zerolog/log"
)

type ListenerConfig struct {
	AppName         string
	ReplaceIfExists bool
	PerProcess      bool
	Verbose         bool
}

func Serve(ctx context.Context, config *ListenerConfig) error {
	sockAddr, err := makeSocketAddr(config.AppName, config.PerProcess)
	if err != nil {
		return err
	}
	ctx, stop := signal.NotifyContext(ctx, os.Interrupt, os.Kill)
	defer stop()

	// Create a unix domain socket for listening
	listener, err := net.Listen(sockAddr.ListenStreamArgs())
	if err != nil {
		log.Error().
			Stringer("address", sockAddr).
			Err(err).
			Msg("Unable to listen on unix socket")

		return err
	}
	defer listener.Close()

	// Serve HTTP on that socket
	go func() {
		_ = http.Serve(
			listener,
			http.HandlerFunc(
				func(w http.ResponseWriter, r *http.Request) {
					bodyBytes, err := io.ReadAll(r.Body)
					if err != nil {
						log.Error().Err(err).Msg("Unable to read request")
						w.WriteHeader(http.StatusInternalServerError)
						return
					}
					fmt.Println(string(bodyBytes))
				},
			),
		)
	}()
	log.Info().Stringer("address", sockAddr).Msg("listening")

	// Wait for SIGINT or SIGKILL
	<-ctx.Done()
	return nil
}
