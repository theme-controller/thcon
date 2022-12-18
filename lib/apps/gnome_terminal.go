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
	"github.com/google/uuid"
	"github.com/gotk3/gotk3/glib"
	"github.com/theme-controller/thcon/lib/event"
	"github.com/theme-controller/thcon/lib/operation"
)

type GnomeTerminalConfig struct {
	GnomeTerminal *struct {
		Disabled bool      `toml:"disabled"`
		Dark     uuid.UUID `toml:"dark" validate:"required_with=Light"`
		Light    uuid.UUID `toml:"light" validate:"required_with=Dark"`
	} `toml:"gnome-terminal"`
}

type GnomeTerminal struct {
	progress event.ProgressChannel
}

func NewGnomeTerminal(progress event.ProgressChannel) Switchable {
	return &GnomeTerminal{
		progress: progress,
	}
}

type windowNode struct {
	XMLName xml.Name `xml:"node"`
	Name    string   `xml:"name,attr"`
}

func listWindows(ctx context.Context, conn *dbus.Conn) ([]windowNode, error) {
	obj := conn.Object("org.gnome.Terminal", "/org/gnome/Terminal/window")
	var windowListXml []byte
	var windowList struct {
		Windows []windowNode `xml:"node"`
	}

	if err := obj.CallWithContext(ctx, "org.freedesktop.DBus.Introspectable.Introspect", 0).Store(&windowListXml); err != nil {
		return nil, fmt.Errorf("Unable to retrieve gnome-terminal widnows from DBus: %+v", err)
	}

	if err := xml.Unmarshal(windowListXml, &windowList); err != nil {
		return nil, fmt.Errorf("Unable to unmarshal XML bytes to windowNode structs: %+v", err)
	}

	return windowList.Windows, nil
}

func setWindowProfile(ctx context.Context, conn *dbus.Conn, windowId string, profileId string) error {
	windowPath := dbus.ObjectPath("/org/gnome/Terminal/window/" + windowId)
	obj := conn.Object("org.gnome.Terminal", windowPath)
	asv := map[string]dbus.Variant{}
	err := obj.CallWithContext(
		ctx,
		"org.gtk.Actions.SetState",
		0,
		"profile",
		dbus.MakeVariant(profileId),
		asv,
	).Store()

	if err != nil {
		return fmt.Errorf("Unable to apply profile '%s' for gnome-terminal window '%s': %+v", profileId, windowId, err)
	}

	return nil
}

func setDefaultProfile(ctx context.Context, profileId string) error {
	glib.SettingsNew(
		"org.gnome.Terminal.ProfilesList",
	).SetString("default", profileId)

	glib.SettingsSync()
	return nil
}

func (gt *GnomeTerminal) ValidateConfig(ctx context.Context, validator *goValidator.Validate, config *Config) error {
	if config.GnomeTerminal == nil {
		return nil
	}

	err := validator.StructCtx(ctx, config.GnomeTerminal)
	var errs *goValidator.ValidationErrors
	if errors.As(err, errs) {
		return *errs
	}

	return nil
}

func (gt *GnomeTerminal) Switch(ctx context.Context, mode operation.Operation, config *Config) error {
	conn, err := dbus.ConnectSessionBus()
	if err != nil {
		return fmt.Errorf("Unable connect to dbus session bus: %+v", err)
	}
	defer conn.Close()

	var profileId string
	switch mode {
	case operation.DarkMode:
		profileId = "235dcfe6-3db0-4f8b-b01c-28e959a3c3ce"
	case operation.LightMode:
		profileId = "efb1da39-d74e-40e4-b21f-d9a7a55fec58"
	}

	gt.progress <- event.AddSubsteps(gt.Name(), 1)
	if err := setDefaultProfile(ctx, profileId); err != nil {
		gt.progress <- event.StepFailed(gt.Name(), err)
		return fmt.Errorf("Unable to set default gnome-terminal profile: %+v", err)
	}
	gt.progress <- event.StepCompleted(gt.Name())

	windows, err := listWindows(ctx, conn)
	if err != nil {
		return err
	}

	if len(windows) > 0 {
		gt.progress <- event.AddSubsteps(gt.Name(), len(windows))
	}

	var setProfileErrors []error
	for _, window := range windows {
		err := setWindowProfile(ctx, conn, window.Name, profileId)
		if err != nil {
			setProfileErrors = append(setProfileErrors, err)
			gt.progress <- event.StepFailed(gt.Name(), err)
		} else {
			gt.progress <- event.StepCompleted(gt.Name())
		}
	}

	if len(setProfileErrors) > 0 {
		var errstrings []string
		for _, err := range setProfileErrors {
			errstrings = append(errstrings, err.Error())
		}
		return errors.New(strings.Join(errstrings, "\n"))
	}

	return nil
}

func (gt *GnomeTerminal) Name() string {
	const name = "Gnome Terminal"
	return name
}

var _ Switchable = (*GnomeTerminal)(nil)
