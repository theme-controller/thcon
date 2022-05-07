# Building `thcon`
1. Install [earthly](https://eathly.dev) on your platform
  1. On Fedora 35 (which supports podman but not docker), I had to add the following block to
     `~/.config/containers/registries.conf`:
     ```toml
     [registries.search]
     registries = ['docker.io']
     ```
2. TBD
