package apps

var All = []Switchable{
	new(MacOS),
	new(TerminalDotApp),
	new(Iterm2),
	NewVim(),
	NewNeovim(),
	new(Alacritty),
	new(Helix),
	new(Kitty),
}
