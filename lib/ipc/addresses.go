package ipc

import (
	"fmt"
	"io/fs"
	"os"
	"os/user"
	"path/filepath"

	"github.com/hashicorp/go-sockaddr"
)

var pid int

func init() {
	pid = os.Getpid()
}

func ensureUserSocketDir() (string, error) {
	currUser, err := user.Current()
	if err != nil {
		return "", fmt.Errorf("Unable to create socket directory for current user: %+v", err)
	}
	dirname := filepath.Join(os.TempDir(), "thcon-"+currUser.Uid)
	err = os.MkdirAll(dirname, 0600)
	if err != nil {
		return "", fmt.Errorf("Unable to create socket directory for current user: %+v", err)
	}

	return dirname, nil
}

func makeSocketAddr(appName string, perProcess bool) (sockaddr.UnixSock, error) {
	dir, err := ensureUserSocketDir()
	if err != nil {
		return sockaddr.UnixSock{}, nil
	}

	var filename string
	if perProcess {
		filename = fmt.Sprintf("%s-%d.sock", appName, pid)
	} else {
		filename = appName + ".sock"
	}
	return sockaddr.NewUnixSock(filepath.Join(dir, filename))
}

func ListSockets(appName string, socketPerProcess bool) (sockaddr.UnixSocks, error) {
	socketDir, err := ensureUserSocketDir()
	if err != nil {
		return nil, err
	}

	if socketPerProcess {
		return listSocketsImpl(os.DirFS(socketDir), socketDir, appName)
	}
	sock, err := sockaddr.NewUnixSock(filepath.Join(socketDir, appName + ".sock"))
	if err != nil {
		return nil, err
	}
	return sockaddr.UnixSocks{ &sock }, nil
}

func listSocketsImpl(fsys fs.FS, basedir string, appName string) (sockaddr.UnixSocks, error) {
	matches, err := fs.Glob(fsys, appName + "-*.sock")
	if err != nil {
		return nil, fmt.Errorf("Failed to list sockets: %+v", err)
	}

	var socks sockaddr.UnixSocks
	for _, path := range matches {
		sock := sockaddr.MustUnixSock(filepath.Join(basedir, path))
		socks = append(socks, &sock)
	}
	return socks, nil
}
