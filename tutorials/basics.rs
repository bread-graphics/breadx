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
//! <div style="border: 2px solid black; border-radius: 12px; margin: 15px; padding: 15px; background: #defffd">
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
//! <div style="border: 2px dotted black; background: #ededed; margin: 15px; padding: 15px">
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
//! TODO: finish
