package operation

type Operation int

const (
	DarkMode Operation = iota
	LightMode
)

func (o Operation) Verb() string {
	const (
		darkStr  = "darkening"
		lightStr = "lightening"
	)
	switch o {
	case DarkMode:
		return darkStr
	case LightMode:
		return lightStr
	}

	return ""
}
