// MIT/Apache2 License

//! Functions to handle `bigreq` functionality.

use super::{prelude::*, Display};
use crate::auto::bigreq::EnableRequest;

#[cfg(feature = "async")]
use super::{
    futures::{ExchangeRequestFuture, MapFuture},
    AsyncDisplay,
};
#[cfg(feature = "async")]
use crate::auto::bigreq::EnableReply;

/// Try to enable `bigreq` for this display.
#[inline]
pub(crate) fn try_bigreq<D: Display + ?Sized>(display: &mut D) -> crate::Result<Option<u32>> {
    match display.exchange_request(EnableRequest::default()) {
        Ok(repl) => Ok(Some(repl.maximum_request_length)),
        Err(crate::BreadError::ExtensionNotPresent(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Try to enable `bigreq` for this display, async redox.
#[cfg(feature = "async")]
#[inline]
pub(crate) fn try_bigreq_async<D: AsyncDisplay + ?Sized>(
    display: &mut D,
) -> MapFuture<
    ExchangeRequestFuture<'_, D, EnableRequest>,
    fn(crate::Result<EnableReply>) -> crate::Result<Option<u32>>,
> {
    MapFuture::run(
        display.exchange_request_async(EnableRequest::default()),
        |repl| match repl {
            Ok(repl) => Ok(Some(repl.maximum_request_length)),
            Err(crate::BreadError::ExtensionNotPresent(_)) => Ok(None),
            Err(e) => Err(e),
        },
    )
}
