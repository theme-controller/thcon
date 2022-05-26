package event

type ProgressKind int

const (
	KAddSubsteps ProgressKind = iota
	KStepStarted
	KStepCompleted
	KStepFailed
)

func (pk ProgressKind) ToString() string {
	switch pk {
	case KAddSubsteps:
		return "add steps"
	case KStepStarted:
		return "start step"
	case KStepCompleted:
		return "finish step"
	case KStepFailed:
		return "fail step"
	default:
		return ""
	}
}

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
		Kind:   KStepStarted,
		Source: source,
	}
}

func AddSubsteps(source string, count int) *ProgressEvent {
	return &ProgressEvent{
		Kind:         KAddSubsteps,
		Source:       source,
		SubstepCount: count,
	}
}

func StepCompleted(source string) *ProgressEvent {
	return &ProgressEvent{
		Kind:   KStepCompleted,
		Source: source,
	}
}

func StepFailed(source string, err error) *ProgressEvent {
	return &ProgressEvent{
		Kind:   KStepFailed,
		Source: source,
		Err:    err,
	}
}

