# FTC HTTP

For pre-compiled binaries, please see the [releases tab](https://github.com/TheLostLambda/ftc_http/releases).
Currently, all three major, desktop operating systems are supported: Linux, Windows, and OSX.
If you are working on something other than a 64-bit, x86 platform, or an operating system not listed above, just open an issue under the [issues tab](https://github.com/TheLostLambda/ftc_http/issues) and I will add a supported binary.

## Configuration
By default, `ftc_http` is set to use a rather aggressive connection timeout
(500ms) when checking for robot controllers on the network. If `ftc_http`
reports that the robot controller is offline when you are certain it's online,
try increasing this value.

When connected via WiFi-Direct, the robot controller listens on one of two IP
addresses:
* `http://192.168.43.1:8080` (REV Control Hub)
* `http://192.168.49.1:8080` (Android Phone)

This version of `ftc_http` automatically tests both addressees, but if your
robot controller is operating on a non-standard host address, you can add it to
the list of hosts checked with the `--host` option.

If the host and timeout options provided yield a successful connection, then
they are automatically remembered and do not need to be given a second time. If
you'd like to reset `ftc_http` to its default configuration, then simply pass
the `--restore_defaults` flag.

## Usage
Short flags can be combined to perform a series of actions following a single
invocation. This somewhat contrived example of this would be the following command:
```
ftc_http -dwub foo/ bar/
```
This command downloads a copy of the code from the robot controller (saving
it in the foo/ directory), wipes the robot controller, uploads a fresh copy
of the code (from the bar/ directory), and builds it.

## Building
To build `ftc_http`, be sure that you have cloned the repository on your computer and then run:

`cargo build --release`

If you do not have Rust / Cargo installed, please see
[rustup.rs](https://www.rustup.rs/).

I've included a `Cargo.lock` file with versions that are known to work, but the
project is unlikely to build without this file. Many dependencies are either
beta versions published only on git, or branches that include some of my own
bug-fixes.

In time, things will stabilise and my bug-fixes should make it into the
upstream. After that, `Cargo.toml` can be populated with proper dependency
versions.

## TODO
- [ ] Stabilise dependencies
- [ ] Document everything
- [ ] Remove cross-compilation stuff from master
