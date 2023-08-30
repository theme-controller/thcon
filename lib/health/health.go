package health

import (
	"context"
	"errors"
	"fmt"
	"reflect"

	goValidator "github.com/go-playground/validator/v10"
)

// Disabled is an struct that must be embedded in any other application config
// struct. Doing so ensures there's a consistent, easy way to allow apps to be
// individually disabled via config file.
type Disabled struct {
	Disabled bool `toml:"disabled"`
}

// IsDisabled returns true if the relevant app config struct was disabled via config file,
// otherwise it returns false.
func (d Disabled) IsDisabled() bool {
	return d.Disabled
}

// Status is the general result of config health checking.
type Status int

func (s Status) String() string {
	switch s {
	case StatusOk:
		return "ok"
	case StatusNotInstalled:
		return "not_installed"
	case StatusMissingConfig:
		return "missing_config"
	case StatusDisabled:
		return "disabled"
	case StatusNotOk:
		return "not_ok"
	case StatusInternalError:
		return "internal_error"
	default:
		return fmt.Sprintf("unimplemented status %d", s)
	}
}

const (
	// StatusOk is returned when the provided app config is healthy, and that
	// the relevant app can be switched.
	StatusOk Status = iota
	// StatusNotInstalled is returned when the application isn't installed.
	// User interaction is required to install the application, but will be
	// silently ignored otherwise.
	StatusNotInstalled
	// StatusMissingConfig is returned when no app config was provided for an
	// app that doesn't (or cannot) have a valid default configuration. User
	// interaction is required.
	StatusMissingConfig
	// StatusDisabled is returned when an app config was provided, but the
	// disabled field was set to 'true'. Valalidation errors -- if any -- are
	// available via the 'error' return value.
	StatusDisabled
	// StatusNotOk is returned when an app is not disabled and has validation
	// errors. The app's theme should not be switched.
	StatusNotOk
	// StatusInternalError is returned when something goes wrong in the
	// implementation of thcon -- typically when a non-nil config struct
	// doesn't embed health.Disabled.
	StatusInternalError
)

// validator is a singleton go-validator instance, to leverage the caching it
// provides.
var validator = goValidator.New()

func init() {
	addCustomValidators(validator)
}

// RequiresConfig rejects a nil config for apps that don't (or cannot) have a
// valid default configuration, and otherwise performs config health checking,
// including:
// * does the config struct embed health.Disabled?
// * are there any validation errors (via go-playground/validator)?
// * is the app disabled via config?
func RequiresConfig(ctx context.Context, cfg interface{}) (Status, error) {
	if reflect.ValueOf(cfg).IsNil() {
		return StatusMissingConfig, nil
	}

	return check(ctx, cfg)
}

// HasDefaults allows a nil config for apps that have a valid default configuration,
// and otherwise performs config health checking, including:
// * does the config struct embed health.Disabled?
// * are there any validation errors (via go-playground/validator)?
// * is the app disabled via config?
func HasDefaults(ctx context.Context, cfg interface{}) (Status, error) {
	if reflect.ValueOf(cfg).IsNil() {
		return StatusOk, nil
	}

	return check(ctx, cfg)
}

var errMissingHealthDisabledEmbed = errors.New("config struct doesn't embed health.Disabled")

// check performs validation on config structs, including whether or not the
// config struct was properly configured, whether the parsed config indicates a
// disabled app, etc.
func check(ctx context.Context, cfg interface{}) (Status, error) {
	isDisabled := reflect.ValueOf(cfg).MethodByName("IsDisabled")
	if !isDisabled.IsValid() {
		return StatusInternalError, errMissingHealthDisabledEmbed
	}

	res := isDisabled.Call([]reflect.Value{})
	if len(res) != 1 || res[0].Kind() != reflect.Bool {
		return StatusInternalError, fmt.Errorf("unexpected result from cfg.IsDisabled(): %+v", res)
	}

	err := validator.StructCtx(ctx, cfg)
	if res[0].Bool() {
		return StatusDisabled, err
	}

	if err != nil {
		return StatusNotOk, err
	}

	return StatusOk, nil
}

// ValidationErrorsToErrorSlice converts a goValidator.ValidationErrors to an
// array of standard Go errors, for easy use with external libraries that
// support arrays of stdlib errors.
func ValidationErrorsToErrorSlice(verrs goValidator.ValidationErrors) []error {
	errs := []error{}
	for _, verr := range verrs {
		errs = append(errs, verr)
	}
	return errs
}
