//go:build linux || dragonfly || freebsd || netbsd || openbsd

package apps

import (
	"context"
	"encoding/xml"
	"errors"
	"fmt"
	"strings"

	goValidator "github.com/go-playground/validator/v10"
	"github.com/godbus/dbus/v5"
	"github.com/rs/zerolog/log"
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

func NewKonsole() Switchable {
	return &Konsole{}
}

func (k *Konsole) Name() string {
	const name = "Konsole"
	return name
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
		return nil, fmt.Errorf("Unable to get names of Konsole services: %+v", err)
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
		return nil, fmt.Errorf("Unable to retrieve konsole sessions: %+v", err)
	}

	if err := xml.Unmarshal(sessionListXml, &sessionList); err != nil {
		return nil, fmt.Errorf("Unable to unmarshal XML bytes to sessionNode structs: %+v", err)
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
		return errors.New("Unable to apply profile")
	}

	return nil
}

func (k *Konsole) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) goValidator.ValidationErrors {
	if config.Konsole == nil {
		return nil
	}

	err := validator.StructCtx(ctx, config.Konsole)
	var errs *goValidator.ValidationErrors
	if errors.As(err, errs) {
		return *errs
	}

	return nil
}

func (k *Konsole) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	conn, err := dbus.ConnectSessionBus()
	if err != nil {
		return fmt.Errorf("Unable connect to dbus session bus: %+v", err)
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
