package apps

import (
	"context"

	"github.com/go-playground/validator/v10"
	"github.com/theme-controller/thcon/lib/operation"
)

type Switchable interface {
	Switch(ctx context.Context, mode operation.Operation, config *Config) error
	Name() string
	// TODO: use https://pkg.go.dev/gopkg.in/validator.v2?utm_source=godoc#Validate
	// to set validation on struct values, then maybe some more advanced checking
	// (e.g. check if profile Foo exists)
	ValidateConfig(ctx context.Context, validator *validator.Validate, config *Config) validator.ValidationErrors
}
