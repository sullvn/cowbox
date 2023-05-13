<h1>
  <div align="center">
    <br />
    <br />
    🐮📦
    <br />
    <br />
    &nbsp;
  </div>
</h1>

Safely run programs without your data
getting borked.

**Cowbox** tricks programs into thinking
they can create/edit/delete files on your
system. In reality, they are working in
a sandbox with your real data.

Available as a command-line tool and Rust
library.

```sh
#
# Without cowbox
#
$ ./strange-script.sh
$ cat my-data
'my-data': No such file or directory

#
# With cowbox
#
$ cowbox exec ./strange-script.sh
$ cat my-data

Krabby Patty Secret Formula
===========================

- King Neptune’s Poseidon Powder
- Whale
- Crab
```


## Use Cases

**Cowbox** is meant to be a time saver when
running random commands or scripts. *Just Run
It*<sup>:tm:</sup> instead of investigating
the details.

Examples:

- Trying out that script from Stack Overflow
- Letting an AI agent loose on your computer
- Blackbox debugging
- Reverse engineering

[Docker][0] and virtual machines are great at
sandboxing programs, but they hide your real
data and require significant setup. And if
you mount your host filesystem in the guest,
then you lose any safety guarantees.

Some operating systems have capability
systems. These are also great, but each
system can require learning, setup, and
modification of files.

Filesystem snapshotting is also awesome,
but is limited in features and isn't a tool
by itself.
 
**Cowbox** is a cross-platform tool which
borrows from all of these alternatives to
make a brainless solution so you can
*Just Run It*<sup>:tm:</sup>.

## Features

- Zero configuration
- Zero system setup
- No administrator privileges required
- Minimal performance overhead
- Supports almost all programs[^1]
- Works on macOS, Linux, and Windows

Not everything is baked yet. See
[Current Limitations](#current-limitations)
and [Roadmap](#roadmap) for details.


## Installation

Pre-built packages are not available yet.

On macOS and Linux:

```sh
$ git clone https://github.com/sullvn/cowbox.git
$ cd cowbox
$ cargo install --path cowbox
```

On Windows (in PowerShell):

```pwsh
> git clone https://github.com/sullvn/cowbox.git
> cd cowbox
> cargo build `
    --release `
    --package cowbox_injection `
    --target i686-pc-windows-msvc `
    --target x86_64-pc-windows-msvc
> cargo install --path cowbox
```


## How It Works

**Cowbox** puts itself between the target
program and your operating system,
intercepting every file access.

Interception is currently done via
dynamic library injection[^2]. This uses the
built-in operating system runtime linker to
re-write parts of the program, in memory, on
program load.

Interception has effectively zero overhead
when done in this way.

A virtual filesystem is created which
records any changes the program makes.

Created files are re-routed to a temporary
place and recorded in the virtual filesystem.
Removed files are marked as removed in the
virtual filesystem, but that is it.

File edits are done on a file copy. In
theory, this can be slow. In practice,
using memory and copy-on-write filesystems
makes this very fast.


## Current Limitations

At the moment, the largest limitation is
lack of universal program support. Only
programs which fulfill the following can be
sandboxed:

- Dynamically linked
- Uses libc (most programs). Or is
  specifically supported by **Cowbox**,
  like Go (in the future).

This is due **Cowbox** using dynamic library
injection. This method is used as
cross-platform baseline, but isn't enough
by itself.

Universal support can be done by utilizing
syscall hooking (Linux, Windows), or binary
re-writing on the instruction level (all
platforms).


## Roadmap

1. Virtual filesystem
2. Pre-built binaries
3. Filesystem tracing interface
4. Support multi-process programs
5. Support shell commands
6. Intercept Go programs
7. Support ARM
8. File and folder whitelisting
9. Syscall hooking on Linux, Windows
10. Network sandboxing
11. Publish to [crates.io][https://crates.io]

## You May Also Like

- [shai][1], a CLI assistant using AI
- [pvw][2], a CLI tool which interactively
  previews command outputs


[^1]: Universal program support is planned.
      See [Current Limitations](#current-limitations)
      for more information.
[^2]: Dynamic library injection has some
      major drawbacks. See
      [Current Limitations](#current-limitations)
      for more information.

[0]: https://www.docker.com
[1]: https://github.com/sullvn/shai
[2]: https://github.com/sullvn/pvw
