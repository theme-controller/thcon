package apps

var All = []Switchable{
	new(MacOS),
	new(TerminalDotApp),
	new(Iterm2),
	NewVim(),
	NewNeovim(),
	new(Helix),
	new(Kitty),
}
