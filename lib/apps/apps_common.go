package apps

import "sort"

func Map() map[string]Switchable {
	out := make(map[string]Switchable)
	for _, app := range All {
		out[app.Argname()] = app
	}
	return out
}

func Names() []string {
	var out []string
	for _, app := range All {
		out = append(out, app.Name())
	}
	sort.Strings(out)
	return out
}

func Argnames() []string {
	var out []string
	for _, app := range All {
		out = append(out, app.Argname())
	}
	sort.Strings(out)
	return out
}
