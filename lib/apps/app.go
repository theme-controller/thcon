package apps

import (
	"context"

	"github.com/theme-controller/thcon/lib/health"
	"github.com/theme-controller/thcon/lib/operation"
)

type Switchable interface {
	Argname() string
	Name() string
	Switch(ctx context.Context, mode operation.Operation, config *Config) error
	ValidateConfig(ctx context.Context, config *Config) (health.Status, error)
}
