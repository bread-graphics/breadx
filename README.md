# breadx

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

## License

Dual licensed under the MIT and Apache 2.0 Licenses, just like Rust proper is.
