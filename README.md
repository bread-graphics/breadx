## Passively Maintained

The original author will spend their effort helping to maintain [`x11rb`](https://crates.io/crates/x11rb) instead of this crate. Bugs will still be fixed; however new features will not be added.

# breadx

[![crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![Build Status][build-badge]][build-url]

[crates-badge]: https://img.shields.io/crates/v/breadx
[crates-url]: https://crates.io/crates/breadx
[docs-badge]: https://img.shields.io/docsrs/breadx
[docs-url]: https://docs.rs/breadx
[build-badge]: https://img.shields.io/github/workflow/status/bread-graphics/breadx/CI
[build-url]: https://github.com/bread-graphics/breadx/actions?query=workflow%3ACI+branch%3Amaster

An implementation of the X Window System Protocol in Rust, with an emphasis on comprehensability and usability.

## Advantages

* `breadx` is simple and direct. There is very little between you and the protocol.
* Without the `sync_display` feature, `breadx` uses no synchronization primitives, eliminating one of the primary causes of deadlocking.
* `breadx` is written in 100% safe code.
* `breadx` is able to be `no_std` and be used without the standard library.
* Runtime-independent `async` support may be enabled using the `async` feature.
* API is designed to be able to be used everywhere.

## Disadvantages

* On its own, `breadx` is not compatible with libraries that use `lixcb` or Xlib. Consider using [`whitebreadx`](https://github.com/bread-graphics/whitebreadx) if this is important.
* `breadx` provides no utility or helper functions beyond the requests that go on the wire.

## Tutorials/Examples

For tutorials and examples of breadx's usage, check out [the docs](https://docs.rs/breadx).

## MSRV

The current MSRV for `breadx` with all features enabled is 1.49.0. This is largely tied to `tokio`'s MSRV, and with fewer features enabled `breadx`
is likely to work on Rust versions as low as 1.46.0 (although this is never guaranteed). The MSRV will not be changed without either a
major or minor version bump.

## License

This package is distributed under the Boost Software License Version 1.0.
Consult the [LICENSE](./LICENSE) file or consult the [web mirror] for
more information.

[web mirror]: https://www.boost.org/LICENSE_1_0.txt
