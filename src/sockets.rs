//! Generates socket addresses on Unix platforms.

use std::path::PathBuf;
use std::process;

/// Returns a PathBuf for an app-specific Unix domain socket, optionally including a process ID for
/// apps that need separate sockets per-instance.
///
/// # Examples
///
/// App `foo` shares an instance of `thcon-listen` across all instances:
///
/// ```no_run
/// # use thcon::sockets::socket_addr;
/// assert_eq!(
///     socket_addr("foo", false),
///     dirs::home_dir()
///         .unwrap()
///         .join(".local/share/thcon/foo.sock"),
/// )
/// ```
///
/// App `bar` requires a new instance of `thcon-listen` for each instance, since it can't share one:
///
/// ```no_run
/// # use thcon::sockets::socket_addr;
/// let pid = std::process::id().to_string();
/// assert_eq!(
///     socket_addr("bar", true),
///     dirs::home_dir()
///         .unwrap()
///         .join(format!(".local/share/thcon/bar/{}.sock", pid)),
/// )
/// ```
pub fn socket_addr(app_name: &str, include_pid: bool) -> PathBuf {
    let mut addr = crate::dirs::temp().join("thcon").join(app_name);

    if include_pid {
        addr.push(process::id().to_string() + ".sock");
    } else {
        addr.set_extension("sock");
    }

    addr
}

#[cfg(not(windows))]
#[test]
fn app_without_pid() {
    assert_eq!(
        dirs::home_dir()
            .unwrap()
            .join(".local/share/thcon/some_app.sock"),
        socket_addr("some_app", false),
    )
}

#[cfg(not(windows))]
#[test]
fn app_with_pid() {
    let pid = process::id().to_string();
    assert_eq!(
        dirs::home_dir()
            .unwrap()
            .join(format!(".local/share/thcon/some_app/{}.sock", pid)),
        socket_addr("some_app", true),
    )
}
