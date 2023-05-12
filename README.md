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

Safely run any program without your data
getting borked.

**Cowbox** tricks programs into thinking
they can create/edit/delete files on your
system. In reality, they are working in
a sandbox.

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


## Features

- Zero configuration
- Zero system setup
- Minimal performance overhead
- Works on macOS, Linux, and Windows


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
