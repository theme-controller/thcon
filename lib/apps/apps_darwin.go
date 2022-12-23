package apps

func All() []Switchable {
	return []Switchable{
		NewMacOS(),
		&TerminalDotApp{},
		&Iterm2{},
		NewVim(),
		NewNeovim(),
		NewHelix(),
	}
}
