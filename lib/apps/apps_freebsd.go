package apps

var All = []Switchable{
	// GNOME
	new(Gtk3),
	new(GnomeShell),
	new(GnomeTerminal),

	// KDE
	new(Konsole),
	new(Plasma),

	// Cross-platform
	new(Emacs),
	NewVim(),
	NewNeovim(),
	new(Alacritty),
	new(Helix),
	new(Kitty),
}
