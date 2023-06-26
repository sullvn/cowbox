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

<div align="center">
  <a href="#current-limitations">
    <img src="https://img.shields.io/badge/-Not%20Ready%20for%20Use-orange?style=for-the-badge" alt="Not Ready for Use" />
  </a>
  <a href="https://github.com/sullvn/cowbox/actions/workflows/test.yaml?query=branch%3Amain">
    <img src="https://img.shields.io/github/actions/workflow/status/sullvn/cowbox/test.yaml?branch=main&label=Tests&style=for-the-badge&logo=github" alt="Tests status" />
  </a>
</div>
<br />
<br />

Safely run programs without your data
getting borked.

*Cowbox* tricks programs into thinking
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

*Cowbox* saves you time when running random
commands or scripts. *Just Run It*
<sup>:tm:</sup> instead of manually
investigating the details.

Examples where sandboxing is useful:

- Trying out that script from Stack Overflow
- Automating tasks with an AI agent
- Blackbox debugging
- Reverse engineering


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

*Cowbox* puts itself between the target
program and your operating system,
intercepting every file access.

Interception is currently done via
dynamic library injection[^2]. This uses the
built-in operating system runtime linker to
re-write parts of the program, in memory, on
program load. This method has almost zero
performance overhead.

A virtual filesystem is created which
records any changes the program makes.

Created files are re-routed to a temporary
place and recorded in the virtual filesystem.
Removed files are merely marked as removed in
the virtual filesystem, but that is it. File
edits are done on a copy of the file. In
theory, copying every file can consume
significant resources. In practice, lazy
copying and modern copy-on-write filesystems
makes this very fast.

Interception can be extended to other program
behavior, such as network access. This is
where *cowbox* will evolve in the future.


## Current Limitations

At the moment, the largest limitation is
lack of universal program support. Only
programs which fulfill the following can be
sandboxed:

- Dynamically linked (most programs)
- Uses libc (most programs). Or is
  specifically supported by *cowbox*,
  like Go (in the future).

This is due to *cowbox* using dynamic
library injection. This method is used as
cross-platform baseline, but isn't enough
by itself.

Universal support can be achieved by
different advanced methods on a
per-platform basis. Examples are OS-assisted
syscall hooks (Linux, Windows) or binary
rewriting (all).

There is also other behavior you may want
sandboxed, such as network or peripheral
access. *Cowbox* does not support this at
the moment, focusing on file access.
With that said, it can naturally be
extended to do so.


## As Compared to X

- [**try**][0] is a simple alternative to
  the `cowbox` CLI if you're on Linux and
  need some basic filesystem sandboxing.
- [**Docker**][1] and VMs virtualize an entire
  OS instead of creating an overlay over your
  own. You can choose to mount your local
  data, but then are at risk of data borking
  once again.
- **File system snapshots** prevent your data
  from being mutilated forever, but leaves
  your files in a broken state until manual
  intervention.
- [**WINE**][2] may seem pretty different,
  but actually it is pretty similar to
  *cowbox*! Both intercept program
  behavior at a low-level, but for different
  purposes. *WINE* creates a Window API
  shim so you can run Windows programs,
  while *cowbox* sandboxes destructive
  operations.


## Roadmap

1. Virtual filesystem
2. Pre-built binaries
3. Filesystem tracing interface
4. Support multi-process programs
5. Support shell commands
6. Intercept Go programs
7. Support ARM
8. File and folder whitelisting
9. Investigate using [`overlayfs`][3] on Linux
10. Syscall hooking on Linux, Windows
11. Network sandboxing
12. Publish to [crates.io][4]


## You May Also Like

- [**shai**][5] – Command-line assistant
  using AI
- [**pvw**][6] – Command-line tool which
  interactively previews command outputs


<div align="center">
  <br />
  <br />
  <br />
  <br />
  🐮📦
  <br />
  <br />
  <br />
  <br />
  &nbsp;
</div>


[^1]: Universal program support is planned.
      See [Current Limitations](#current-limitations)
      for more information.
[^2]: Dynamic library injection has some
      major drawbacks. See
      [Current Limitations](#current-limitations)
      for more information.

[0]: https://github.com/binpash/try
[1]: https://www.docker.com
[2]: https://www.winehq.org
[3]: https://docs.kernel.org/filesystems/overlayfs.html
[4]: https://crates.io
[5]: https://github.com/sullvn/shai
[6]: https://github.com/sullvn/pvw
