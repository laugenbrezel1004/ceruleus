
use std::fs::File;

use daemonize::Daemonize;

pub fn run() -> Result<(), String> {
    //journalctl by systemd?

    let daemonize = Daemonize::new()
        .pid_file("/tmp/ceruleus.pid") // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/") // for default behaviour.
        // TODO:
        .user("nobody")
        .group("daemon") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");


    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }
    Ok(())
    //
    //
    // match daemonize.start() {
    //     Ok(_) => {
    //         println!("Success, daemonized");
    //         Ok(())
    //     }
    //     Err(e) => {
    //         eprintln!("Error, {}", e);
    //         Err(e.to_string())
    //     }
    // }
    //         Err(e.to_string())
    //     }
    // }
    // some kind of loop, with signalhandeling
}
