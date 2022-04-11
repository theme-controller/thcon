package apps

import (
	"context"

	"github.com/theme-controller/thcon/lib/operation"
)

type RootConfig struct{}

type Switchable interface {
	Switch(ctx context.Context, mode operation.Operation, config *RootConfig) error
}
