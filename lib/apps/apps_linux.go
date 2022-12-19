package apps

func All() []Switchable {
	return []Switchable{
		// GNOME
		NewGtk3(),
		NewGnomeShell(),
		NewGnomeTerminal(),

		// KDE
		NewKonsole(),
		NewPlasma(),

		// Cross-platform
		NewVim(),
		NewNeovim(),
		NewHelix(),
	}
}
