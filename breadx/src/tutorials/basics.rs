//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

//! # Basic Usage of `breadx`
//!
//! [Last Tutorial]
//!
//! Let's create a simple application using `breadx`. I'll explain each of
//! the pieces along the way.
//!
//! [Last Tutorial]: crate::tutorials::introduction
//!
//! ## Application Setup
//!
//! First, let's just create a simple application that depends on `breadx`.
//!
//! TODO: application setup once breadx is on crates.io
//!
//! You should see a `main.rs` file. This is the entry point for your
//! application. It's a good place to put your main logic.
//!
//! ## Connect to the Server
//!
//! The first thing you do is connect to the X11 server. You can use the
//! [`DisplayConnection::connect()`] method, which connects to the server.
//!
//! ```rust,no_run
//! use breadx::{display::DisplayConnection, Result};
//!
//! fn main() -> Result<()> {
//!     let mut connection = DisplayConnection::connect(None)?;
//!     Ok(())
//! }
//! ```
//!
//! For those unfamiliar with Rust, I'll go into each part of the program:
//!
//! - `use breadx::{...}` imports the [`DisplayConnection`] and [`Result`]
//!   types from the `breadx` crate.
//! - `fn main() -> Result<()>` defines the main function, which is the
//!   entry point for the application. It returns [`Result`], which is a
//!   [sum type] consisting of the success state (`Ok`) and the failure
//!   state (`Err`).
//! - `let mut connection = ...` defines the `connection` variable,
//!   makes it a [mutable] variable, and initializes it to the result of
//!   the following expression.
//! - `DisplayConnection::connect(...)` creates a new [`DisplayConnection`]
//!   that is connected to the currently running X11 server. It opens a
//!   TCP or Unix connection, preforms the initial setup handshake, and
//!   then returns a type that can be used to communicate with the X11
//!   server.
//! - `None` is a version of another sum type, [`Option`]. In this case,
//!   it is a parameter passed to [`DisplayConnection::connect()`]. The
//!   first parameter of the function indicates the *display name* to be
//!   used, which determines what X11 server to connect to. In this case,
//!   `None` indicates to use the display name contained in the [`DISPLAY`]
//!   environment variable.
//! - The `?` at the end is syntactic sugar. `connect()` returns a [`Result`],
//!   and if it is `Err`, the `?` operator will cause `main()` to return the
//!   error early.
//! - `Ok(())` at the end is used to tell the function to return the success
//!   status, since we made it through all right. The `()` is the [empty tuple],
//!   which is often used in Rust to express that we have no data.
//!
//! To summarize, all this does is open the connection to the X11 server,
//! and then immediately closes it. That won't do. How about we do something
//! with it?
//!
//! [`DisplayConnection`]: crate::display::DisplayConnection
//! [`Result`]: std::result::Result
//! [sum type]: https://en.wikipedia.org/wiki/Tagged_union
//! [`DISPLAY`]: https://askubuntu.com/a/919913
//! [mutable]: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
//! [`DisplayConnection::connect()`]: crate::display::DisplayConnection::connect
//! [empty tuple]: https://doc.rust-lang.org/std/primitive.unit.html
//! [`Option`]: std::option::Option
//!
//! ## Create a Window
//!
//! Let's send our first request. There's pretty much nothing better we can
//! do than create a window at this point.
//!
//! There are a few things we have to take care of first. We need to determine
//! the "parent" of the window, or the window that it's inside of. We'll go more
//! in depth as to what exactly a window is earlier, but you should know that the
//! actual screen can be used as a parent.
//!
//! ```rust,no_run
//! use breadx::{display::DisplayConnection, prelude::*, protocol::xproto, Result};
//!
//! fn main() -> Result<()> {
//!     let mut connection = DisplayConnection::connect(None)?;
//!     let parent = connection.default_screen().root;
//!     Ok(())
//! }
//! ```
//!
//! Some things to take note of:
//!
//! - Note that we are now importing the [prelude], which contains
//!   certain [traits] to enhance the functionality of the
//!   [`DisplayConnection`] type.
//! - We also import the [`xproto`] module, which contains some useful
//!   types that we'll use later.
//! - We set parent to the value of `root` of [`connection.default_screen()`],
//!   which returns the screen we're assigned to use by default.
//!
//! [prelude]: https://doc.rust-lang.org/reference/names/preludes.html
//! [traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
//! [`DisplayConnection`]: crate::display::DisplayConnection
//! [`connection.default_screen()`]: crate::display::DisplayBase::default_screen
//! [`xproto`]: crate::protocol::xproto
//!
//! Now that everything is in place, we can create the window.
//!
//! ```rust,no_run
//! use breadx::{display::DisplayConnection, prelude::*, protocol::xproto, Result};
//!
//! fn main() -> Result<()> {
//! #    let mut connection = DisplayConnection::connect(None)?;
//! #    let parent = connection.default_screen().root;
//!     // snip existing code     
//!
//!     let wid = connection.generate_xid()?;
//!     let window = connection.create_window(
//!         0, // depth
//!         wid,
//!         parent,
//!         0, // x
//!         0, // y
//!         600, // width
//!         400, // height
//!         0, // border width
//!         xproto::WindowClass::COPY_FROM_PARENT,
//!         0, // visual class
//!         xproto::CreateWindowAux::new(),
//!     )?;
//!
//!     Ok(())
//! }
//! ```
//!
//! First, using the [`generate_xid()`] function, we generate a unique *XID*. This is
//! a value that the client and server will share to identify resources. XIDs
//! aren't just unique to the process, they're unique across the entire system. This
//! means that you can use XIDs that originate in other processes, and they will
//! still be unique.
//!
//! Then, we call the [`create_window()`] function. This function takes the following
//! parameters:
//!
//! - The depth of the window. This is the number of bits per pixel. Setting it
//!   to `0` means that the window will use the depth of its parent.
//! - The window ID. This is the value that the client and server will share to
//!   identify the window.
//! - The parent window. This is the window that the new window will be inside of.
//!   Remember that `parent` refers to the screen.
//! - The x and y coordinates of the window.
//! - The width and height of the window.
//! - The border width. This is the width of the border around the window. However,
//!   you don't actually have a border around a top-level window, so this value doesn't
//!   really matter.
//! - The *window class*, which determines whether the window is an "input only" window
//!   or an "input-output" window. The distinction isn't important for this example,
//!   so we just copy it from the parent.
//! - The *visual class* defines certain properties of the window, like the range of colors
//!   that are available. `0` once again indicates that we need to inherit from the parent.
//! - [`CreateWindowAux`] is used to contain properties that are not strictly necessary for
//!   creating a window, but are still useful. In this case, we don't need to specify any
//!   properties, so we just use the default constructor.
//!
//! [`generate_xid()`]: crate::display::Display::generate_xid
//! [`create_window()`]: crate::display::DisplayFunctionsExt::create_window
//! [`CreateWindowAux`]: crate::protocol::xproto::CreateWindowAux
//!
//! If you run this code, you'll notice that nothing happens. You see, while the
//! window is created, it's not actually visible. We need to tell the server to
//! actually show the window.
