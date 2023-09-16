//go:build linux || dragonfly || freebsd || netbsd || openbsd

package apps

import (
	"context"
	"encoding/xml"
	"errors"
	"fmt"
	"strings"

	"github.com/godbus/dbus/v5"
	"github.com/rs/zerolog/log"
	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
)

type KonsoleConfig struct {
	Konsole *struct {
		Disabled bool   `toml:"disabled"`
		Dark     string `toml:"dark" validate:"required_with=Light"`
		Light    string `toml:"light" validate:"required_with=Dark"`
	} `toml:"konsole"`
}

type Konsole struct{}

func (k *Konsole) Name() string {
	const name = "Konsole"
	return name
}

func (k *Konsole) Argname() string {
	const argname = "konsole"
	return argname
}

func getKonsoleServiceIds(ctx context.Context, conn *dbus.Conn) ([]string, error) {
	obj := conn.Object("org.freedesktop.DBus", "/")
	var names []string
	err := obj.CallWithContext(
		ctx,
		"ListNames",
		0,
	).Store(&names)

	if err != nil {
		return nil, fmt.Errorf("unable to get names of Konsole services: %+v", err)
	}

	var konsoles []string
	for _, name := range names {
		if strings.HasPrefix(name, "org.kde.konsole-") {
			konsoles = append(konsoles, name)
		}
	}

	return konsoles, nil
}

func getSessionIds(ctx context.Context, conn *dbus.Conn, serviceId string) ([]string, error) {
	type SessionNode struct {
		XMLName xml.Name `xml:"node"`
		Name    string   `xml:"name,attr"`
	}
	type SessionList struct {
		Sessions []SessionNode `xml:"node"`
	}

	obj := conn.Object(serviceId, "/Sessions")
	var sessionListXml []byte
	var sessionList SessionList

	if err := obj.CallWithContext(ctx, "org.freedesktop.DBus.Introspectable.Introspect", 0).Store(&sessionListXml); err != nil {
		return nil, fmt.Errorf("unable to retrieve konsole sessions: %+v", err)
	}

	if err := xml.Unmarshal(sessionListXml, &sessionList); err != nil {
		return nil, fmt.Errorf("unable to unmarshal XML bytes to sessionNode structs: %+v", err)
	}

	var sessions []string
	for _, session := range sessionList.Sessions {
		sessions = append(sessions, session.Name)
	}

	return sessions, nil
}

func applyProfile(ctx context.Context, conn *dbus.Conn, serviceId string, sessionId string, profileName string) error {
	sessionPath := dbus.ObjectPath("/Sessions/" + sessionId)
	obj := conn.Object(serviceId, sessionPath)
	err := obj.CallWithContext(
		ctx,
		"org.kde.konsole.Session.setProfile",
		0,
		profileName,
	).Store()

	if err != nil {
		return errors.New("unable to apply profile")
	}

	return nil
}

func (k *Konsole) ValidateConfig(ctx context.Context, config *Config) (health.Status, error) {
	return health.RequiresConfig(ctx, config.Konsole)
}

func (k *Konsole) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	conn, err := dbus.ConnectSessionBus()
	if err != nil {
		return fmt.Errorf("unable connect to dbus session bus: %+v", err)
	}
	defer conn.Close()

	var profileName string
	switch mode {
	case operation.DarkMode:
		profileName = "Profile 1"
	case operation.LightMode:
		profileName = "zipper"
	}

	konsoleServices, err := getKonsoleServiceIds(ctx, conn)
	if err != nil {
		return err
	}
	log.Debug().
		Strs("service", konsoleServices).
		Msg("list services")

	var getSessionErrs []error
	for _, service := range konsoleServices {
		sessions, err := getSessionIds(ctx, conn, service)
		if err != nil {
			log.Error().
				Str("service", service).
				Err(err).
				Msg("list sessions")
			getSessionErrs = append(getSessionErrs, err)
			continue
		}

		serviceLogger := log.With().
			Str("service", service).
			Strs("sessions", sessions).
			Logger()
		serviceLogger.Debug().Msg("list sessions")

		for _, session := range sessions {
			err := applyProfile(ctx, conn, service, session, profileName)
			if err != nil {
				serviceLogger.Error().
					Str("profile", profileName).
					Err(err).
					Msg("apply profile")
			}
		}
	}

	if len(getSessionErrs) > 0 {
		var errStrings []string
		for _, err := range getSessionErrs {
			errStrings = append(errStrings, err.Error())
		}
		return errors.New(strings.Join(errStrings, "\n"))
	}

	return nil
}
