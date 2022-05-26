package ipc

import (
	"bytes"
	"context"
	"net"
	"net/http"

	"github.com/hashicorp/go-sockaddr"
	"github.com/rs/zerolog/log"
)

type Outbound struct {
	Socket  *sockaddr.UnixSock
	Message []byte
}

func Send(ctx context.Context, payload *Outbound) error {
	log.Debug().Stringer("addr", payload.Socket).Msg("send")

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
