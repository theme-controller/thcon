package ipc

import (
	"bytes"
	"context"
	"net"
	"net/http"

	"github.com/apex/log"
	"github.com/hashicorp/go-sockaddr"
)

type Outbound struct {
	Socket  *sockaddr.UnixSock
	Message []byte
}

func Send(ctx context.Context, payload *Outbound) error {
	logger := log.FromContext(ctx)
	logger.WithField("addr", payload.Socket.String()).Debug("send")

	c := http.Client{
		Transport: &http.Transport{
			DialContext: func(_ctx context.Context, _network string, _addr string) (net.Conn, error) {
				return net.Dial(payload.Socket.DialStreamArgs())
			},
		},
	}
	res, err := c.Post("http://_", "application/json", bytes.NewReader(payload.Message))
	if err != nil {
		return err
	}
	defer res.Body.Close()
	return nil
}
