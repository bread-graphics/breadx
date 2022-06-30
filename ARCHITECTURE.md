# Architecture of `breadx`

This file aims to describe the way that `breadx` is structured. This should, hopefully, act as a catalogue of potential starting points for new contributors.

This document assumes some knowledge of the core X11 protocol. Consider reading [this document](https://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.pdf) for more information.

## Dependency Justification

Optional dependencies are *italic*, and public dependencies are **bold**.

- [`advance`] is an "unsafe-quarantine microcrate" that I created that
  essentially reimplements [`IoSlice::advance()`] in a way that is safe for use in non-Nightly environments. It consists of about four lines
  of code that maneuvers around lifetimes requirements.
- [`ahash`] is used to provide a fast, hardware optimized collision
  resistant hash function for the extension map.
- ***[`async-io`]*** is the async-I/O OS driver backing the 
  [`async-std`] and [`smol`] runtimes. It is optionally imported when 
  the `async-std-support` feature is enabled so that 
  [`async_io::Async::<impl CanBeAsyncDisplay>`] can implement
  [`AsyncDisplay`].
- *[`blocking`]* is the blocking-tasks thread pool used by the    
  [`async-std`] and [`smol`] runtimes. It is optionally imported when 
  the `async-std-support` feature is enabled, in order to isolate
  the loading of `.Xauthority` onto the thread pool.
- **[`bytemuck`]** is a public dependency, since the [`NoUninit`] trait
  is used as a bound for the [`Void`] trait. It is exclusively used
  in the [`Void`] trait and could likely be removed in the future.
- [`cfg_if`] is used to branch between features at various points in an
  idiomatic way. This could be removed in the future.
- *[`concurrent-queue`]* is used to create a list of wakers that is used
  to handle asynchronous access to the `Mutex` in [`SyncDisplay`]. It
  is only enabled when the `sync_display` feature is.
- *[`fionread`]* is an unsafe-quarantine microcrate that I created that 
  wraps around the `FIONREAD` ioctl call. For the Windows target, it is
  used to tell when the `non_blocking_*` family of functions should
  continue on with the read or exit early. This dependency could likely
  be removed in the future if a better way of preforming non-blocking
  reads on Windows is implemented.
- *[`futures-util`]* is imported when the `async` feature is enabled,
  and is primarily used to provide convenience functions on the
  [`Future`] trait and import the [`Stream`] trait.
- *[`gethostname`]* is imported when the `std` feature is enabled, and is
  used to get the hostname of the current computer for use in the
  authorization protocol.
- [`hashbrown`] is used to provide the hash map used in the extension 
  map. Note that this is the only place in `breadx` where hash maps are
  used.
- *[`nix`]* is used to implement certain functionality for connections
  on Unix targets. Namely, it is used for sending ancillary file
  descriptors along the wire for named connections. It is only imported
  when the `std` feature is enabled, and is an empty crate on non-Unix
  targets.
- *[`parking_lot`]* is used to replace the mutexes normally used in
  [`SyncDisplay`] when the `pl` feature is enabled. Note that the
  standard library now uses a "futex" implementation that is
  supposedly faster than [`parking_lot`]'s locking mechanisms, so this
  feature may be removed in the future.
- *[`socket2`]* is used to initialize abstract sockets for the Unix 
  target, initialize sockets in a non-blocking manner for `async`,
  and provide a generic shutdown implementation for [`StdConnection`].
  It is only used when the `std` feature is enabled.
- ***[`tokio`]*** is used so that [`AsyncDisplay`] can be implemented
  for [`AsyncFd::<impl CanBeAsyncDisplay>`]. Only the `net` and `rt`
  features are enabled. `net` is necessary for the [`AsyncFd`] type,
  and `rt` is used to provide [`spawn_blocking`] so that file loading
  work can be offset onto a blocking thread. This crate is only imported
  when the `tokio-support` feature is enabled.
- [`tracing`] is used to provide logging support. As an aside, 
  performance could probably be increased by restricting certain log
  emissions to debug mode.
- **[`x11rb-protocol`]** is used to provide the X11 protocol. Various
  modules of [`x11rb-protocol`] are re-exported, including the 
  `protocol` and `x11_utils` modules. In order to prevent breaking
  changes from occurring (as [`x11rb-protocol`] is an unstable crate),
  it is pinned to a patch version.

[`advance`]: https://crates.io/crates/advance
[`IoSlice::advance()`]: https://doc.rust-lang.org/std/io/struct.IoSlice.html#method.advance
[`ahash`]: https://crates.io/crates/ahash
[`async-io`]: https://crates.io/crates/async-io
[`async-std`]: https://crates.io/crates/async-std
[`smol`]: https://crates.io/crates/smol
[`async_io::Async::<impl CanBeAsyncDisplay>`]: https://docs.rs/async-io/latest/async_io/struct.Async.html
[`AsyncDisplay`]: https://docs.rs/breadx/latest/breadx/display/trait.AsyncDisplay.html
[`blocking`]: https://crates.io/crates/blocking
[`bytemuck`]: https://crates.io/crates/bytemuck
[`NoUninit`]: https://docs.rs/bytemuck/latest/bytemuck/trait.NoUninit.html
[`Void`]: https://docs.rs/breadx/latest/breadx/trait.Void.html
[`cfg_if`]: https://crates.io/crates/cfg_if
[`concurrent-queue`]: https://crates.io/crates/concurrent-queue
[`SyncDisplay`]: https://docs.rs/breadx/latest/breadx/display/struct.SyncDisplay.html
[`futures-util`]: https://crates.io/crates/futures-util
[`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html
[`gethostname`]: https://crates.io/crates/gethostname
[`hashbrown`]: https://crates.io/crates/hashbrown
[`nix`]: https://crates.io/crates/nix
[`parking_lot`]: https://crates.io/crates/parking_lot
[`socket2`]: https://crates.io/crates/socket2
[`tokio`]: https://crates.io/crates/tokio
[`tokio-support`]: https://crates.io/crates/tokio-support
[`tracing`]: https://crates.io/crates/tracing
[`x11rb-protocol`]: https://crates.io/crates/x11rb-protocol
[`StdConnection`]: https://docs.rs/breadx/latest/breadx/connection/struct.StdConnection.html
[`AsyncFd`]: https://docs.rs/tokio/latest/tokio/io/unix/struct.AsyncFd.html
[`AsyncFd::<impl CanBeAsyncDisplay>`]: https://docs.rs/tokio/latest/tokio/io/unix/struct.AsyncFd.html
[`spawn_blocking`]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html

## Overall Structure

This is the overall structure of the `breadx` repository, from the
repository root.

### `breadx/`

This contains the actual `breadx` crate that is published on crates.io.

### `breadx-generator/`

This generator is used to create the `automatically_generated.rs` file,
which contains helper functions for sending requests to the X server.
It uses the `xcbproto` submodule (also in the repository root) as a
source of information to generate from.

### `xcb_parser/`

This library is a parser for the XML-XCB format used in `xcbproto`. It
is used by `breadx-generator` to parse the `xcbproto` submodule.

### `breadx/src/connection/`

The `connection` module defines the [`Connection`] trait, which is
used to represent the "byte stream" that is used to communicate with
the X server. It can be seen as a `no_std`-compatible version of
[`Read`] and [`Write`], with extra extensions to support file
descriptor passing and non-blocking reading. [`BasicDisplay`] uses
this type as a transport.

In addition to the [`Connection`] trait, there are three other important
types that are defined here:

- [`BufConnection`], which acts as a wrapper over another [`Connection`]
  type, but adds buffering to both ends of the stream. It is analogous
  to [`BufReader`] and [`BufWriter`].
- [`StdConnection`] wraps around a [`Read`] + [`Write`] type to create
  a [`Connection`] type. Note that, for the extra features, the type
  also needs to be [`AsRawFd`] on Unix systems and [`AsRawSocket`] on
  Windows ones.
- [`SendmsgConnection`] wraps around a [`UnixStream`] and provides a
  variant of it that can pass file descriptors.

The user should be using [`NameConnection`] most of the time and
shouldn't need to worry about this module.

[`Connection`]: https://docs.rs/breadx/latest/breadx/connection/trait.Connection.html
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`BasicDisplay`]: https://docs.rs/breadx/latest/breadx/display/struct.BasicDisplay.html
[`BufConnection`]: https://docs.rs/breadx/latest/breadx/connection/struct.BufConnection.html
[`StdConnection`]: https://docs.rs/breadx/latest/breadx/connection/struct.StdConnection.html
[`SendmsgConnection`]: https://docs.rs/breadx/latest/breadx/connection/struct.SendmsgConnection.html
[`NameConnection`]: https://docs.rs/breadx/latest/breadx/connection/struct.NameConnection.html
[`AsRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html
[`AsRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawSocket.html
[`UnixStream`]: https://docs.rs/tokio/latest/tokio/net/unix/struct.UnixStream.html

### `breadx/src/display/`

The `display` module contains all of the inner workings of the
connections to the X11 server. This contains the most complex code
by far.

The root module provides four primary traits:

- [`DisplayBase`] provides common functionality between all types
  of connections, like being able to poll for replies/events as
  well as the [`Setup`]. Note that the `Setup` is in an `Arc` in order
  to allow it to be kept outside of a mutex for things like 
  [`SyncDisplay`].
- [`Display`] is a blocking connection.
- [`CanBeAsyncDisplay`] is something that *could* be a non-blocking
  connection, if it were given an I/O driver.
- [`AsyncDisplay`] is a non-blocking connection complete with an
  attached I/O driver.

[`DisplayBase`]: https://docs.rs/breadx/latest/breadx/display/trait.DisplayBase.html
[`Display`]: https://docs.rs/breadx/latest/breadx/display/trait.Display.html
[`CanBeAsyncDisplay`]: https://docs.rs/breadx/latest/breadx/display/trait.CanBeAsyncDisplay.html
[`AsyncDisplay`]: https://docs.rs/breadx/latest/breadx/display/trait.AsyncDisplay.html
[`SyncDisplay`]: https://docs.rs/breadx/latest/breadx/display/struct.SyncDisplay.html
[`Setup`]: https://docs.rs/breadx/latest/breadx/display/struct.Setup.html

### `breadx/src/display/basic.rs`

This module contains the implementation of the [`BasicDisplay`], which
is the canonical implementation of the [`Display`] trait.

[`BasicDisplay`]: https://docs.rs/breadx/latest/breadx/display/struct.BasicDisplay.html
[`Display`]: https://docs.rs/breadx/latest/breadx/display/trait.Display.html

### `breadx/src/display/cell.rs`

TODO: finish this