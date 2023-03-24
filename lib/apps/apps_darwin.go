package apps

var All = []Switchable{
	NewMacOS(),
	&TerminalDotApp{},
	&Iterm2{},
	NewVim(),
	NewNeovim(),
	NewHelix(),
	NewKitty(),
}
