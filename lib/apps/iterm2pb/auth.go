//go:build darwin

package iterm2pb

import (
	"context"
	"fmt"
	"os/exec"
	"strings"
)

// getCookieAndKey uses 'osascript' to request an authentication cookie and
// app-identifying key for iTerm2 RPC via websockets.
//
// See https://gitlab.com/gnachman/iterm2/-/issues/9058#note_448782729
// See https://gitlab.com/gnachman/iterm2/-/wikis/iTerm2-Version-3.3-Security-Updates
func getCookieAndKey(ctx context.Context) (string, string, error) {
	out, err := exec.CommandContext(
		ctx,
		"osascript",
		"-e",
		`tell application "iTerm2" to request cookie and key for app named "thcon"`,
	).CombinedOutput()
	if err != nil {
		return "", "", fmt.Errorf("unable to get iTerm2 cookie and key (%s) - %v", string(out), err)
	}

	outStr := strings.TrimSpace(string(out))
	cookie, key, found := strings.Cut(outStr, " ")
	if !found {
		return "", "", fmt.Errorf("received malformed cookie and key: %q", outStr)
	}
	return cookie, key, nil
}
