package event

type ProgressKind int

const (
	addSubsteps ProgressKind = iota
	stepStarted
	stepCompleted
	stepFailed
)

type ProgressEvent struct {
	Kind         ProgressKind
	SubstepCount int
	Source       string
	Message      string
	Err          error
}

type ProgressChannel chan *ProgressEvent

func StepStarted(source string) *ProgressEvent {
	return &ProgressEvent{
		Kind:   stepStarted,
		Source: source,
	}
}

func AddSubsteps(source string, count int) *ProgressEvent {
	return &ProgressEvent{
		Kind:         addSubsteps,
		Source:       source,
		SubstepCount: count,
	}
}

func StepCompleted(source string) *ProgressEvent {
	return &ProgressEvent{
		Kind:   stepCompleted,
		Source: source,
	}
}

func StepFailed(source string, err error) *ProgressEvent {
	return &ProgressEvent{
		Kind:   stepFailed,
		Source: source,
		Err:    err,
	}
}
