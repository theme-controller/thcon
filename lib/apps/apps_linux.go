package apps

import "github.com/theme-controller/thcon/lib/event"

func All(progressChan event.ProgressChannel) []Switchable {
	return []Switchable{
		// GNOME
		NewGtk3(),
		NewGnomeShell(),
		NewGnomeTerminal(progressChan),

		// KDE
		NewKonsole(),
		NewPlasma(),

		// Cross-platform
		NewNeovim(progressChan),
	}
}
