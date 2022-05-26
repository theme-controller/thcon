package util

import (
	"time"

	"github.com/rs/zerolog"
)

func RecordDuration() DurationHook {
	return DurationHook {
		start: zerolog.TimestampFunc(),
	}
}

type DurationHook struct {
	start time.Time
}

func (dh DurationHook) Run(e *zerolog.Event, level zerolog.Level, message string) {
	e.Dur("duration", time.Since(dh.start))
}
