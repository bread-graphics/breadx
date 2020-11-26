// MIT/Apache2 License

//! [In the last tutorial](../intro/index.html), we reached an elementary understanding of how the X protocol works. In this
//! tutorial, we'll open a connection to the X server and create a window.
//!
//! # Setting up the Environment
//!
//! We'll use a basic Rust executable environment to build our examples. First, we create the project folder
//! by running the following command:
//!
//! ```bash
//! $ cargo new --bin example
//! $ cd example
//! ```
//!
//! Then, we configure the `Cargo.toml` file to automatically download `breadx` and its dependencies. Under the
//! `[dependencies]` header in the `Cargo.toml` file, add:
//!
//! ```plaintext
//! breadx = "0.1.0"
//! ```
//!
//! # Writing the Program
//!
//! With this current setup, anything we write in `src/main.rs` will be executed whenever the crate is compiled
//! and run. Let's see what we have to do.
//!
//! The [`DisplayConnection`](../../display/type.DisplayConnection.html) object is the liason to the X11 server.
//! Not only does it create and manage the connection, it stores information regarding the hardware and server.
//! To create the object, one must call [`DisplayConnection::create`](../../display/type.DisplayConnection.html#method.create).
//!
//! *In src/main.rs*
//!
//! ```no_run
//! use breadx::{BreadError, DisplayConnection};
//!
//! fn main() -> Result<(), BreadError> {
//!     let conn = DisplayConnection::create(None, None);
//!     
//!     Ok(())
//! }
//! ```
//!
//! <div style="border: 2px solid black; border-radius: 12px; margin: 15px; padding: 15px; background: #defffd">
//! <h2>Dissecting `DisplayConnection::create`</h2>
//!
//! Above, we pass in two `None` parameters to `DisplayConnection::create`. In most cases, this should work
//! properly. However, it does do some good to know what, exactly, each parameter is doing.
//!
//! The first parameter is the **server name**. This is an address pointing to the X11 server that the program
//! should connect to. It has the following format:
//!
//! ```plaintext
//! [protocol/][host]:display[.screen]
//! ```
//!
//! The default system server name is stored in the `DISPLAY` environment variable. On most systems, it should
//! just be `:0`, representing the first display server. `None` tells `breadx` to read from the `DISPLAY`
//! variable and use that value as the server name. You shouldn't ever have to change this.
//!
//! The second parameter is the authentication info. **TODO: EXPLAIN**
//! </div>
//!
//! <div style="border: 2px dotted black; margin: 15px; padding: 15px; background: #daffd9">
//! <h2>Another Perspective on `DisplayConnection`</h2>
//!
//! Shrewd programmers will check the documentation and realize that `DisplayConnection` is actually a
//! type alias.
//!
//! ```rust,no_compile
//! type DisplayConnection = Display<NameConnection>;
//! ```
//!
//! [`Display`](../../display/struct.Display.html) is a general purpose wrapper around any type of connection.
//! This allows X connections to be held across every type of byte stream. Most of the time, programmers
//! will simply use the [`NameConnection`](display/name/enum.NameConnection.html) connection, which is a wrapper
//! around TCP Streams or Unix sockets on compatible systems. However, the option remains to run X on any
//! object that implements [`Connection`](../../display/trait.Connection.html).
//! </div>
//!
//! Running this program opens the X connection, then immediately closes it. This isn't that useful. With our
//! connection, we can open a window using the `create_simple_window` method.
//!
//! TODO: finish
