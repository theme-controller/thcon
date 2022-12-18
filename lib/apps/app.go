package apps

import (
	"context"
	"errors"

	"github.com/go-playground/validator/v10"
	"github.com/theme-controller/thcon/lib/operation"
)

// ErrNeedsConfig indicates that an application requires configuration, but
// that configuration wasn't provided.
var ErrNeedsConfig = errors.New("app needs configuration")

type Switchable interface {
	Switch(ctx context.Context, mode operation.Operation, config *Config) error
	Name() string
	ValidateConfig(ctx context.Context, validator *validator.Validate, config *Config) error
}
