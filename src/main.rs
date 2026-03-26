use std::{
    env,
    os::unix::process::ExitStatusExt,
    process::{Command, exit},
};

const SIGPIPE: i32 = 13;
const USAGE: &str = "
pyper [command] [arguments...]

Pyper is a simple command line utility for dealing with SIGPIPE signals on
Linux.

It wraps another command and silently swallows the SIGPIPE signal, returning
gracefully.

If the wrapped command exits normally, pyper exits with the same status code.
If the wrapped command is killed by a signal other than SIGPIPE, pyper prints
the signal code to stderr and exits with status 2.
";

fn main() {
    let mut args = env::args();
    args.next().expect("pyper: the program doesn't have a name");
    let command_name = args.next().unwrap_or(String::from("-h"));

    if command_name == "-h" || command_name == "--help" {
        print!("{}", USAGE);
        return;
    }

    let mut command = Command::new(command_name);
    while let Some(arg) = args.next() {
        command.arg(arg);
    }

    let status = command.status().expect("pyper: failed to execute command");
    if let Some(signal) = status.signal() {
        if signal == SIGPIPE {
            exit(0);
        } else {
            eprintln!("pyper child process exited with signal {}", signal);
            exit(2);
        }
    }

    if let Some(code) = status.code() {
        exit(code);
    };

    eprintln!(
        "pyper child process returned unrecognized result: {:?}",
        status
    );
    exit(1);
}
