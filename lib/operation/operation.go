package operation

type Operation string

const (
	DarkMode  Operation = "dark"
	LightMode Operation = "light"
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

func (o Operation) String() string {
	return string(o)
}
