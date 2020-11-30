# breadx

[![Build Status](https://dev.azure.com/jtnunley01/gui-tools/_apis/build/status/not-a-seagull.breadx?branchName=master)](https://dev.azure.com/jtnunley01/gui-tools/_build/latest?definitionId=11&branchName=master) [![crates.io](https://img.shields.io/crates/v/breadx)](https://crates.io/crates/breadx) [![Docs](https://docs.rs/breadx/badge.svg)](https://docs.rs/breadx)

An implementation of the X Window System Protocol in Rust. 100% safe and mutex-free. Currently a work in progress; however, it should be usable in its current state for most X11-related applications.

## Reasons you should use this over Xlib/XCB Bindings

* No Mutexes
* Generally faster (awaiting verification)
* Built-in support for Rust's async ecosystem via `async_net`
* Crate proper is `#[forbid(unsafe_code)]`, dependencies are either safe or verified.
* Tries to provide the ease of use of Xlib while also providing XCB's ability to leverage the async capabilities of the X server
* Can be used in `#[no_std]` environments.

## Reasons not to use this over Xlib/XCB Bindings

* Currently very immature
* No support for extensions (yet)
* Not ABI/API compatible with Xlib/XXB

## Tutorials/Examples

For tutorials and examples of breadx's usage, check out [the docs](https://docs.rs/breadx).

## License

Dual licensed under the MIT and Apache 2.0 Licenses, just like Rust proper is.
