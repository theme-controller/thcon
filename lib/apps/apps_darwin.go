package apps

func All() []Switchable {
	return []Switchable{
		NewMacOS(),
		&TerminalDotApp{},
		NewVim(),
		NewNeovim(),
		NewHelix(),
	}
}
