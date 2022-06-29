// MIT/Apache2 License

//! Futures that are used throughout the program.
//!
//! In `breadx`, we manually create our own futures instead of using
//! `async` syntax, in order to allow traits to return either thread-safe or
//! thread-unsafe futures, based on the flavor of [`Display`] being used.
//! These futures are stored here.
//!
//! [`Display`]: crate::display::Display

mod check_for_error;
pub use check_for_error::*;

mod checked;
pub use checked::*;

mod flush;
pub use flush::*;

mod generate_xid;
pub use generate_xid::*;

mod send_request;
pub use send_request::*;

mod send_request_raw;
pub use send_request_raw::*;

mod synchronize;
pub use synchronize::*;

mod try_with;
pub use try_with::*;

mod wait_for_event;
pub use wait_for_event::*;

mod wait_for_reply;
pub use wait_for_reply::*;

mod wait_for_reply_raw;
pub use wait_for_reply_raw::*;
