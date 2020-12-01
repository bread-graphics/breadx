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
//!     let mut conn = DisplayConnection::create(None, None)?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! <div class="dissecting">
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
//! <div class="another-perspective">
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
//! ```no_run
//! use breadx::{BreadError, DisplayConnection};
//!
//! fn main() -> Result<(), BreadError> {
//!     let mut conn = DisplayConnection::create(None, None)?;
//!
//!     // the root, or parent, refers to the overall screen
//!     let root = conn.default_root();
//!
//!     // represents the colors black and white, respectively
//!     let black = conn.default_black_pixel();
//!     let white = conn.default_white_pixel();
//!     
//!     // creates a 640x400 window with a black border and a white background
//!     let window = conn.create_simple_window(root, 0, 0, 640, 400, 0, black, white)?;
//!
//!     // ensure the window is mapped to the screen. this operation is equivalent to the
//!     // "show" function in other GUI toolkits
//!     window.map(&mut conn)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! <div class="dissecting">
//! <h2>Dissecting <a href="../../display/struct.Display.html#method.create_simple_window">`Display::create_simple_window`</a></h2>
//!
//! The `create_simple_window` method acts as a wrapper around the [`Display::create_window`](../../display/struct.Display.html#method.create_window) method, that passes in some defaults that assume values based on the parent
//! window. You may have to use `Display::create_window` for cases where the visual type or depth of the window
//! won't be the same as its parent.
//! </div>
//!
//! If you compile and run this program, you will see a window breifly appear on your screen before vanishing
//! in a second. This is because it creates the window, maps the window, but then immediately ends the program.
//! In order to ensure the window persists, you need to create an event loop.
//!
//! # The Event Loop
//!
//! Event loops should be familiar concept to those who've used GTK+ or Node.js. In X, it's a lot more literal:
//! it's just a loop that reads in events, processes them, and then waits for the next event. To provide an
//! analogy, consider the following Rust program:
//!
//! ```no_run
//! use std::{error::Error, io::{prelude::*, stdin, stdout}};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // hold a lock on Stdin and Stdout for convenience
//!     let ins = stdin();
//!     let outs = stdout();
//!     let mut buffer = String::with_capacity(10);
//!
//!     loop {
//!         // write a prompt to the user
//!         outs.write_all(b"Input your favorite number: ")?;
//!         outs.flush();
//!
//!         // read in user input
//!         ins.read_line(&mut buffer)?;
//!
//!         // pop off the newline at the end
//!         buffer.pop();
//!
//!         // try to parse it into something we can understand
//!         // assume that a failed number parse is a sentinel to close
//!         let input = match buffer.parse::<i32>() {
//!             Err(_) => {
//!                 outs.write_all(b"Exiting...\n")?;
//!                 break Ok(());
//!             }
//!             Ok(input) => input,
//!         };
//!
//!         // write interpretation of input to user
//!         if input & 1 == 0 {
//!             outs.write_all(b"Number is even.\n")?;
//!         } else {
//!             outs.write_all(b"Number is odd.\n")?;
//!         }
//!
//!         // loop back around and do it again
//!     }
//! }
//! ```
//!
//! This is a relatively standard program that reads in a number from the user, and determines whether it's
//! odd or even. The key point here is that is runs indefinitely; even after just one input, it keeps going.
//!
//! Now, consider:
//!
//! * Replacing the prompt to the user with the creation of the window and the connection, and moving it
//!   outside of the loop (we've already done this!).
//! * Replacing reading in the user input with reading in the event from the connection.
//! * Replacing the parsing of the input with the parsing of the event.
//! * Replacing writing the interpretation of the inputs of the user with modifying the window setup.
//!
//! This, in a nutshell, is the X event loop.
//!
//! ```no_run
//! use breadx::{BreadError, DisplayConnection};
//!
//! fn main() -> Result<(), BreadError> {
//!     // simplified form of the above
//!     let mut conn = DisplayConnection::create(None, None)?;
//!     let window = conn.create_simple_window(
//!         conn.default_root(),
//!         0,
//!         0,
//!         640,
//!         400,
//!         0,
//!         conn.default_black_pixel(),
//!         conn.default_white_pixel(),
//!     )?;
//!
//!     window.map(&mut conn)?;
//!
//!     // begin the event loop
//!     loop {
//!         let _ev = conn.wait_for_event()?;
//!         // TODO: do things with the event
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! You should now see a blank window. Depending on your WM and GUI environment, it may look like this:
//!
//! <img src="https://raw.githubusercontent.com/not-a-seagull/breadx/master/images/basics_window.png" />
//!
//! Congratulations! You should be able to move it around, resize it, and what have you. It's a bit bare at
//! the moment, however. Let's see if we can fix that.
//!
//! # Adding a window title
//!
//! The [`Window::set_title`](../../auto/xproto/struct.Window.html#method.set_title) function allows
//! the user to set the window's title.
//!
//! ```no_run
//! use breadx::{BreadError, DisplayConnection};
//!
//! fn main() -> Result<(), BreadError> {
//!     // simplified form of the above
//!     let mut conn = DisplayConnection::create(None, None)?;
//!     let window = conn.create_simple_window(
//!         conn.default_root(),
//!         0,
//!         0,
//!         640,
//!         400,
//!         0,
//!         conn.default_black_pixel(),
//!         conn.default_white_pixel(),
//!     )?;
//!
//!     // NEW: set the window's title
//!     window.set_title(&mut conn, "Hello World!")?;
//!
//!     window.map(&mut conn)?;
//!
//!     // begin the event loop
//!     loop {
//!         let _ev = conn.wait_for_event()?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! <div class="note">
//! <h2>Note</h2>
//!
//! You may notice that most functions rely on a mutable reference to the connection. This is because all data
//! coming in and out of the client must go through the connection.
//! </div>
//!
//! Running this program adds a title to your window:
//!
//! <img src="https://raw.githubusercontent.com/not-a-seagull/breadx/master/images/basics_title.png" />
//!
//! # Setting the exit protocol
//!
//! At this point, you may have noticed that pressing the "x" button on the window does not close the window.
//! The only way to close it is to kill the process. This is because X predates the concept of a window
//! manager. We have to register the idea of a WM deletion process with the client, and use that to close the
//! program. **TODO: explain this better**
//!
//! We do this by registering the `WM_DELETE_WINDOW` atom and checking for it in every iteration of the event
//! loop.
//!
//! ```no_run
//! use breadx::{BreadError, Event, DisplayConnection};
//!
//! fn main() -> Result<(), BreadError> {
//!     // simplified form of the above
//!     let mut conn = DisplayConnection::create(None, None)?;
//!     let window = conn.create_simple_window(
//!         conn.default_root(),
//!         0,
//!         0,
//!         640,
//!         400,
//!         0,
//!         conn.default_black_pixel(),
//!         conn.default_white_pixel(),
//!     )?;
//!     window.set_title(&mut conn, "Hello World!")?;
//!     window.map(&mut conn)?;
//!
//!     // NEW: intern an atom and set it to the window's WM protocol
//!     let wm_delete_window = conn
//!         .intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false)?;
//!     window.set_wm_protocols(&mut conn, &[wm_delete_window])?;
//!
//!     loop {
//!         let ev = conn.wait_for_event()?;
//!
//!         // NEW: match the event, see if it's a ClientMessageEvent, and
//!         // exit the program if it holds the delete window atom
//!         // interned above
//!         match ev {
//!             Event::ClientMessage(cme) => {
//!                 if cme.data.longs()[0] == wm_delete_window.xid() {
//!                     break;
//!                 }
//!             }
//!             _ => (),
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! <div class="another-perspective">
//! <h2>Another Perspective on the `_immediate` suffix</h2>
//!
//! You may notice that several functions in this library have alternatives with the `_immediate` suffix.
//! `_immediate` means that this function resolves its result immediately, which means it:
//!
//! 1). Sends a request to the server.
//!
//! 2). Waits for a reply from the server.
//!
//! 3). Parses the reply from the server.
//!
//! This may seem like the obvious thing to do, and the question is: "why isn't every function in the library
//! immediate?" The answer to that lies in a subtlety of how X's asynchronous nature works. Consider if we
//! didn't have to intern just one atom, as above, but several.
//!
//! ```no_run
//! # use breadx::{Atom, BreadError, DisplayConnection};
//! # fn main() -> Result<(), BreadError> {
//! # let mut conn = DisplayConnection::create(None, None)?;
//! let atom_names: Vec<String> = vec![ /* ... */ ];
//! let mut atoms: Vec<Atom> = Vec::with_capacity(atom_names.len());
//!
//! for aton_name in atom_names {
//!     atoms.push(conn.intern_atom_immediate(atom_name, false)?);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Consider the workflow for the above program. It would:
//!
//! * Send `InternAtomRequest` #1 to the server.
//! * Wait for reply #1.
//! * Send `InternAtomRequest` #2 to the server.
//! * Wait for reply #2.
//! * and so on and so forth.
//!
//! Although this is an *acceptable* solution, we can do better. We could send all of the requests in a batch,
//! and then wait for replies after the requests have been sent. This would look like:
//!
//! ```no_run
//! # use breadx::{Atom, BreadError, DisplayConnection};
//! # fn main() -> Result<(), BreadError> {
//! # let mut conn = DisplayConnection::create(None, None)?;
//! use breadx::{auto::xproto::InternAtomRequest, RequestCookie};
//!
//! let atom_names: Vec<String> = vec![ /* ... */ ];
//! let mut atom_cookies: Vec<RequestCookie<InternAtomRequest>> = Vec::with_capacity(atom_names.len());
//! let mut atoms: Vec<Atom> = Vec::with_capacity(atom_names.len());
//!
//! for atom_name in atom_names {
//!     // send the request to the server
//!     atom_cookies.push(conn.intern_atom(atom_name, false)?);
//! }
//!
//! for atom_cookie in atom_cookies {
//!     // resolve the request from the server.
//!     atoms.push(conn.resolve_request(atom_cookie)?.atom);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! The above workflow is now:
//!
//! * Send `InternAtomRequest` #1 to the server.
//! * Send `InternAtomRequest` #2 to the server.
//! * and so on and so forth.
//! * Wait for reply #1.
//! * Wait for reply #2.
//! * and so on and so forth.
//!
//! This tends to be much more efficient. Of course, if you're only interning one atom, then
//! `intern_atom_immediate` if preferred.
//! </div>
//!
//! You should notice that the window now closes when the "x" button is clicked. Splendid!
//!
//! [Next time, we'll take a closer look at events and how we can use them to implement functionality in
//! our program.](../event/index.html)
