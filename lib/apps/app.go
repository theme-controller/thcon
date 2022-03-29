package apps

import (
	"github.com/theme-controller/thcon/lib/operation"
)

type Switchable interface {
	Switch(mode operation.Operation, config interface{}) error
}
