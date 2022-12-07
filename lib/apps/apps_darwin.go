package apps

import "github.com/theme-controller/thcon/lib/event"

func All(progressChan event.ProgressChannel) []Switchable {
	return []Switchable{
		NewMacOS(),
		// Terminal.app is intentionally disabled for now
		// while I investigate AppleScript failures on 12.5.1.
		// &TerminalDotApp{},
		NewVim(progressChan),
		NewNeovim(progressChan),
		NewHelix(),
	}
}
