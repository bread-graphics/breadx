# breadx\_generator

This program parses the [XML-XCB](https://xcb.freedesktop.org/XmlXcb/) specification to produce 100% safe Rust code to be used to interface with the X11 server. This code is stored in breadx's `src/auto` directory and used as a data format during communications with the server.

Similar to most compilers, breadx\_generator uses several "levels" of representation to make optimizations and moficiation easier:

* **Level 0** - Refers to the XML format present in the `xml` directory of the repository root.
* **Level 1** - A structural format very similar to the XML format.
* **Level 2** - A normalized structural format using higher-level concepts than Level 1. Most optmizations are done here.
* **Level 3** - A structural format similar to the Rust output code. Generally translates directly to `syn` expressions.
* **Level 4** - `syn` expressions.
* **Level 5** - Output text.

This program is automatically built and run over files using the `Makefile` in the repository root. Consider using that instead of `cargo` operations on this crate.

## Translation (Top-Level Elements)

### `<import>`

The `import` tag translates to a wildcard import of its header name:

```
use super::<header_name>::*;
```

### `<struct>`

Translates to a Rust structure that is clonable and has a default value. It also generates an implementation of `AsByteSequence` for the structure to serialize it to or deserialize it from a series of bytes. See `Structure Contents` below for information on how the contents are translated.

```
#[derive(Debug, Clone, Default)]
pub struct <identifier> {
    // struct. contents
} 
```
