package apps

import "github.com/theme-controller/thcon/lib/event"

func All(progressChan event.ProgressChannel) []Switchable {
	return []Switchable{
		NewVim(progressChan),
		NewNeovim(progressChan),
	}
}
