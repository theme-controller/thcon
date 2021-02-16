use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process;

#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;

use async_std::os::unix::net::UnixListener;
use async_std::prelude::*;
use async_std::task;

use signal_hook::consts::signal::*;
use signal_hook_async_std::Signals;

use clap::{crate_version, App, AppSettings, Arg};

async fn remove_socket(sock_path: PathBuf, verbose: bool) -> Result<()> {
    let result = async_std::fs::remove_file(&sock_path).await;
    if verbose {
        match result {
            Ok(_) => eprintln!("Succesfully removed socket at '{}'", sock_path.display()),
            Err(e) => eprintln!("Failed to remove socket at '{}': {}", sock_path.display(), e),
        }
    }

    return Ok(());
}

async fn on_exit<'a>(signals: Signals, sock_path: PathBuf, verbose: bool) -> Result<()> {
    let mut signals = signals.fuse();
    while let Some(signal) = signals.next().await {
        match signal {
            SIGTERM | SIGINT | SIGQUIT => return remove_socket(sock_path, verbose).await,
            _ => unreachable!(),
        }
    }
    Ok(())
}

async fn listen<'a>(sock_path: PathBuf, verbose: bool) -> Result<()> {
    let listener = match UnixListener::bind(&sock_path).await {
        Ok(listener) => {
            if let Err(err) = fs::metadata(&sock_path).and_then(|md| {
                let mut perms = md.permissions();
                perms.set_mode(0o700);
                fs::set_permissions(&sock_path, perms)
            }) {
                let sock_name = sock_path.to_str().unwrap();
                eprintln!(
                    "Could not change permissions of socket at '{}': {}",
                    sock_name, err
                );
                remove_socket(sock_path, verbose).await?;
                process::exit(exitcode::NOPERM);
            }
            listener
        }
        Err(err) => {
            let sock_name = sock_path.to_str().unwrap();
            eprintln!(
                "Could not listen to unix-domain socket at '{}': {}",
                sock_name, err
            );

            remove_socket(sock_path, verbose).await?;
            process::exit(exitcode::CANTCREAT);
        }
    };

    if verbose {
        let sock_name = sock_path.to_str().unwrap();
        eprintln!("Listening to unix-domain socket at '{}'", sock_name);
    }

    let mut incoming = listener.incoming();
    let mut request = String::new();
    while let Some(stream) = incoming.next().await {
        match stream {
            Ok(mut stream) => {
                request.clear();
                match stream.read_to_string(&mut request).await {
                    Ok(_) => {
                        let maybe_json: serde_json::Result<serde_json::Value> =
                            serde_json::from_str(&request.trim());
                        if let Ok(json_input) = maybe_json {
                            println!("{}", json_input);
                        } else if verbose {
                            eprintln!("Received non-JSON input: '{}'", &request)
                        }
                    }
                    Err(e) => {
                        eprintln!("Encountered error while reading from stream: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Encountered error while reading from stream: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    task::block_on(async {
        let argv = App::new("thcon-listen")
                    .version(crate_version!())
                    .author("Sean Barag <sean@barag.org>")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .arg(Arg::with_name("verbose")
                            .short("v")
                            .long("verbose")
                            .help("Enables verbose output")
                    ).arg(Arg::with_name("per_process")
                            .long("per-process")
                            .help("Creates a connection unique to this process, rather than one connection for the entire app")
                    ).arg(Arg::with_name("name")
                            .help("The name of the app listening to this connection")
                            .takes_value(true)
                            .required(true)
                    ).get_matches();

        #[cfg(windows)]
        unimplemented!();

        let is_verbose = argv.is_present("verbose");

        let sock_path = thcon::sockets::socket_addr(
            argv.value_of("name").unwrap(),
            argv.is_present("per_process"),
        );
        let sock_dir = sock_path.parent().unwrap();
        if is_verbose {
            eprintln!("sock_path = {}", sock_path.display());
            eprintln!("sock_dir = {}", sock_dir.display());
        }

        if !sock_dir.exists() {
            if is_verbose {
                eprintln!("Creating socket directory {}", sock_dir.display());
            }
            if let Err(e) = fs::create_dir_all(&sock_dir) {
                eprintln!(
                    "Could not create socket directory {}: {}",
                    sock_dir.display(),
                    e
                );
                process::exit(exitcode::IOERR);
            }
        }

        if sock_path.exists() {
            if is_verbose {
                eprintln!("Removing pre-existing (stale?) socket {}", sock_path.display());
            }
            fs::remove_file(&sock_path).unwrap_or_default();
        }

        let signals = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])?;
        let signal_stream = signals.handle();
        let signals_task = task::spawn(on_exit(signals, sock_path.clone(), is_verbose));

        task::spawn(listen(sock_path.clone(), is_verbose));

        signals_task.await?;
        signal_stream.close();

        Ok(())
    })
}
