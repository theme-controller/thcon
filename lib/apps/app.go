package apps

import (
	"context"

	"github.com/theme-controller/thcon/lib/operation"
)

type Switchable interface {
	Switch(ctx context.Context, mode operation.Operation, config *Config) error
	Name() string
}
