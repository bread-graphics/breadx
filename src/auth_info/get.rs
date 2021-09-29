// MIT/Apache2 License

use super::{family, AuthInfo};
use itoa::Buffer;
use std::{io::Error, ops::ControlFlow};

#[cfg(feature = "async")]
use futures_lite::{Stream, StreamExt};

#[inline]
pub(crate) fn get_auth(
    mut auths: impl Iterator<Item = Result<AuthInfo, Error>>,
    family: u16,
    address: &[u8],
    display: u16,
) -> Result<Option<AuthInfo>, Error> {
    let mut buffer = Buffer::new();
    let display = buffer.format(display);

    // TODO: use try_find() once it is stabilized
    match auths.try_fold((), |(), entry| match entry {
        Ok(entry)
            if addr_match(family, address, entry.family, &entry.address)
                && display_match(&entry.number, display.as_bytes())
                && entry.name == b"MIT-MAGIC-COOKIE-1" =>
        {
            ControlFlow::Break(Ok(entry))
        }
        Ok(_) => ControlFlow::Continue(()),
        Err(e) => ControlFlow::Break(Err(e)),
    }) {
        ControlFlow::Break(Ok(b)) => Ok(Some(b)),
        ControlFlow::Break(Err(e)) => Err(e),
        ControlFlow::Continue(()) => Ok(None),
    }
}

#[cfg(feature = "async")]
#[inline]
pub(crate) async fn get_auth_async(
    mut auths: impl Stream<Item = Result<AuthInfo, Error>> + Unpin,
    family: u16,
    address: &[u8],
    display: u16,
) -> Result<Option<AuthInfo>, Error> {
    let mut buffer = Buffer::new();
    let display = buffer.format(display);

    // TODO: use try_find() once it is stabilized
    match auths
        .try_for_each(|entry| match entry {
            Ok(entry)
                if addr_match(family, address, entry.family, &entry.address)
                    && display_match(&entry.number, display.as_bytes())
                    && entry.name == b"MIT-MAGIC-COOKIE-1" =>
            {
                Err(Ok(entry))
            }
            Ok(_) => Ok(()),
            Err(e) => Err(Err(e)),
        })
        .await
    {
        Err(Ok(b)) => Ok(Some(b)),
        Err(Err(e)) => Err(e),
        Ok(()) => Ok(None),
    }
}

#[inline]
fn addr_match(f1: u16, a1: &[u8], f2: u16, a2: &[u8]) -> bool {
    if f1 == family::WILD || f2 == family::WILD {
        true
    } else if f1 != f2 {
        false
    } else {
        a1 == a2
    }
}

#[inline]
fn display_match(en: &[u8], dn: &[u8]) -> bool {
    en.is_empty() || en == dn
}
