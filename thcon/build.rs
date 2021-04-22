use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        mac: { target_os="macos" },
        dbus: { any(
            target_os="linux",
            target_os="freebsd",
            target_os="dragonfly",
            target_os="openbsd",
            target_os="netbsd"
        ) },
    }
}
