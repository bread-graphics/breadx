// MIT/Apache2 License

//! This module contains futures necessary to run the async versions of displays.
//!
//! You may reasonably be asking, "notgull, why have you manually coded about a dozen futures instead of using
//! async/await syntax". The answer is "I need to return these from traits". At the moment, you cannot have an
//! async function in a trait in Rust, because the type system isn't powerful enough to represent the higher
//! kinded type that would be needed to express the future.
//!
//! Currently, the main solution is to return a `Pin<Box<dyn Future>>`. However, this is a problem when it comes
//! to thread safety. There are some displays that are thread safe and those that aren't. If we decide to return
//! a `Pin<Box<dyn Future + Send>>>`, we prevent ourselves from using non thread-safe displays. If we decide to
//! return a `Pin<Box<dyn Future>>` (which is `!Send`), then users can't spawn our futures on thread-safe
//! runtimes, which limits where we can use them. The only solution is to manually code all of our futures, since
//! the generics on these futures dictates whether or not they are thread safe.

mod and_then;
mod either;
mod exchange_request;
mod looping;
mod map;
mod put_image;
mod read_packet;
mod resolve_request;
mod resolve_request_raw;
mod send_packet;
mod send_request;
mod send_request_raw;
mod synchronize;
mod wait;
mod wait_for_event;
mod wait_for_special_event;

pub use and_then::ExchangeXidFuture;
pub use either::EitherFuture;
pub use exchange_request::ExchangeRequestFuture;
pub use looping::{WaitLoopFuture, WaitLoopHandler};
pub use map::MapFuture;
pub use put_image::PutImageFuture;
pub use read_packet::ReadPacketFuture;
pub use resolve_request::ResolveRequestFuture;
pub use resolve_request_raw::{ResolveRequestRawFuture, ResolveRequestRawHandler};
pub use send_packet::SendPacketFuture;
pub use send_request::SendRequestFuture;
pub use send_request_raw::SendRequestRawFuture;
pub use synchronize::SynchronizeFuture;
pub use wait::WaitFuture;
pub use wait_for_event::{WaitForEventFuture, WaitForEventHandler};
pub use wait_for_special_event::{WaitForSpecialEventFuture, WaitForSpecialEventHandler};
