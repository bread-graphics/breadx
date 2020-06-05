# flutterbug

A basic set of X11 bindings. This is intended to be used as a comprehensive, safe interface to the X Window System. Of note is that this interface uses Xlib instead of XCB, because:

1). I know Xlib better than XCB.

2). I couldn't find a good set of Rust bindings to XCB.

This code is dual licensed under the MIT License and the Apache 2.0 License. See the `examples` directory for usage.
