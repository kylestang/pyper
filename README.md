# Pyper

*Pipe without fear*

We've all been there. You're writing the next groundbreaking, revolutionary, world-changing bash script when, suddenly, `exit 141`. What happened? You, being a responsible script wizard, ran `set -euo pipefail` at the start. Sadly, a command with a lot of output was piped into a command that only needed a little. The reader closed the pipe, leaving the writing command to be mercilessly `SIGPIPE`'d by the operating system. The pipe fails. The script fails. Your hopes and dreams fail.

But fear not! The woeful cries of tens of StackOverflow users have finally been answered. `pyper` wraps your garrulous source command and gracefully catches the dreaded `SIGPIPE`, leaving stdin, stdout, and the exit code untouched. At long last, your scripts may complete happily ever after.

## Usage

```
pyper [command] [arguments...]

Pyper is a simple command line utility for dealing with SIGPIPE signals on
Linux.

It wraps another command and silently swallows the SIGPIPE signal, returning
gracefully.

If the wrapped command exits normally, pyper exits with the same status code.
If the wrapped command is killed by a signal other than SIGPIPE, pyper prints
the signal code to stderr and exits with status 2.
```

Bash example:
```bash
set -euo pipefail

# Misery
yes | head -n 1

# Joy
pyper yes | head -n 1
```

## Install

`cargo install pyper`
