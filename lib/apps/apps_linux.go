package apps

var All = []Switchable{
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
