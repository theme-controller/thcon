// go:build darwin
package iterm2pb

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"path/filepath"

	"github.com/hashicorp/go-sockaddr"
	"nhooyr.io/websocket"
	"nhooyr.io/websocket/wspb"
)

type Client struct {
	Conn *websocket.Conn
	Res  *ServerOriginatedMessage
}

func NewClient(ctx context.Context) (*Client, error) {
	// Authenticate via the OS.
	cookie, key, err := getCookieAndKey(ctx)
	if err != nil {
		return nil, fmt.Errorf("unable to create iTerm2 websocket client: %v", err)
	}

	// Build the path to the unix socket hosting the websocket connection.
	homedir, err := os.UserHomeDir()
	if err != nil {
		return nil, fmt.Errorf("unable to find iterm2 socket: %v", err)
	}

	// err is never nil for NewUnixSock.
	sock, _ := sockaddr.NewUnixSock(filepath.Join(homedir, "Library/Application Support/iTerm2/private/socket"))

	// Build an HTTP client for the iTerm2 websocket.
	sockClient := http.Client{
		Transport: &http.Transport{
			DialContext: func(_ context.Context, _ string, _ string) (net.Conn, error) {
				return net.Dial(sock.DialStreamArgs())
			},
		},
	}

	// Set headers required for authentication.
	// See https://gitlab.com/gnachman/iterm2/-/wikis/iTerm2-Version-3.3-Security-Updates#authentication
	headers := http.Header{}
	headers.Set("origin", "ws://localhost")
	headers.Set("x-iterm2-library-version", "go 3.7")
	headers.Set("x-iterm2-cookie", cookie)
	headers.Set("x-iterm2-key", key)

	// Finally, connect to the iTerm2 websocket.
	conn, res, err := websocket.Dial(ctx, "ws://localhost", &websocket.DialOptions{
		HTTPClient:   &sockClient,
		HTTPHeader:   headers,
		Subprotocols: []string{"api.iterm2.com"},
	})
	if err != nil {
		var body string
		if res != nil {
			bodyBytes, _ := io.ReadAll(res.Body)
			body = string(bodyBytes)
		}
		return nil, fmt.Errorf("unable to make websocket client. res: %q, err: %v", body, err)
	}

	// Increase the maximum size of readable responses beyond the default 32kB.
	conn.SetReadLimit(1024 * 1024 * 5)

	return &Client{Conn: conn, Res: new(ServerOriginatedMessage)}, nil
}

func (c *Client) ListProfiles(ctx context.Context) ([]*ListProfilesResponse_Profile, error) {
	defer c.Res.Reset()

	req := ClientOriginatedMessage{
		Submessage: &ClientOriginatedMessage_ListProfilesRequest{
			ListProfilesRequest: &ListProfilesRequest{},
		},
	}

	if err := wspb.Write(ctx, c.Conn, &req); err != nil {
		return nil, fmt.Errorf("error sending list profiles request: %v", err)
	}
	if err := wspb.Read(ctx, c.Conn, c.Res); err != nil {
		return nil, fmt.Errorf("error reading list profiles response: %v", err)
	}

	if err := c.Res.GetError(); err != "" {
		return nil, fmt.Errorf("received error response from iTerm2: %v", err)
	}

	res := c.Res.GetListProfilesResponse()
	if res == nil {
		return nil, fmt.Errorf("received unexpected response type from iTerm2: %s", res)
	}
	profiles := res.GetProfiles()
	if profiles == nil {
		return nil, errors.New("got response from iTerm2, but with nil profiles")
	}
	c.Res.Reset()
	return res.GetProfiles(), nil
}

func (c *Client) SetDefaultProfile(ctx context.Context, guid string) error {
	defer c.Res.Reset()

	req := ClientOriginatedMessage{
		Submessage: &ClientOriginatedMessage_PreferencesRequest{
			PreferencesRequest: &PreferencesRequest{
				Requests: []*PreferencesRequest_Request{
					{
						Request: &PreferencesRequest_Request_SetDefaultProfileRequest{
							SetDefaultProfileRequest: &PreferencesRequest_Request_SetDefaultProfile{
								Guid: &guid,
							},
						},
					},
				},
			},
		},
	}

	if err := wspb.Write(ctx, c.Conn, &req); err != nil {
		return fmt.Errorf("error sending set default profile request: %v", err)
	}
	if err := wspb.Read(ctx, c.Conn, c.Res); err != nil {
		return fmt.Errorf("error reading set default profile response: %v", err)
	}

	if err := c.Res.GetError(); err != "" {
		return fmt.Errorf("received error response from iTerm2: %v", err)
	}

	return nil
}

func (c *Client) GetSessionIds(ctx context.Context) ([]string, error) {
	defer c.Res.Reset()

	req := ClientOriginatedMessage{
		Submessage: &ClientOriginatedMessage_ListSessionsRequest{
			ListSessionsRequest: &ListSessionsRequest{},
		},
	}
	if err := wspb.Write(ctx, c.Conn, &req); err != nil {
		return nil, fmt.Errorf("error sending list sessions request: %v", err)
	}

	if err := wspb.Read(ctx, c.Conn, c.Res); err != nil {
		return nil, fmt.Errorf("error reading list sessions response: %v", err)
	}

	if err := c.Res.GetError(); err != "" {
		return nil, fmt.Errorf("received error response from iTerm2: %v", err)
	}

	res := c.Res.GetListSessionsResponse()

	sessionIds := []string{}
	for _, window := range res.GetWindows() {
		for _, tab := range window.GetTabs() {
			for _, link := range tab.GetRoot().Links {
				sessionIds = append(sessionIds, link.GetSession().GetUniqueIdentifier())
			}
		}
	}

	for _, buried := range res.GetBuriedSessions() {
		buried.GetUniqueIdentifier()
	}

	return sessionIds, nil
}

func (c *Client) UpdateCurrentProfileInSessions(ctx context.Context, profileProps map[string]string, sessionIds []string) error {
	// Build a reusable Assignments struct that we'll pass to each session,
	// effectively copying the settings from the found profile onto each active
	// session.
	assignments := []*SetProfilePropertyRequest_Assignment{}
	for k, v := range profileProps {
		k := k
		v := v
		assignments = append(assignments, &SetProfilePropertyRequest_Assignment{
			Key:       &k,
			JsonValue: &v,
		})
	}

	for _, sessionId := range sessionIds {
		req := ClientOriginatedMessage{
			Submessage: &ClientOriginatedMessage_SetProfilePropertyRequest{
				SetProfilePropertyRequest: &SetProfilePropertyRequest{
					Target: &SetProfilePropertyRequest_Session{
						Session: sessionId,
					},
					Assignments: assignments,
				},
			},
		}
		res := new(ServerOriginatedMessage)

		if err := wspb.Write(ctx, c.Conn, &req); err != nil {
			return fmt.Errorf("error sending set profile property request for session %s: %v", sessionId, err)
		}

		if err := wspb.Read(ctx, c.Conn, res); err != nil {
			return fmt.Errorf("error reading set profile property response for sessino %s: %v", sessionId, err)
		}

		if err := res.GetError(); err != "" {
			return fmt.Errorf("received error response from iTerm2: %s", err)
		}
	}

	return nil
}
