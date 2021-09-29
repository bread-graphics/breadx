// MIT/Apache2 License

// Note: a lot of the auth code is stolen from x11rb, thanks psychon!

pub(crate) mod family;

#[cfg(feature = "std")]
mod file;
#[cfg(feature = "std")]
mod get;
#[cfg(feature = "std")]
mod reader;

use alloc::vec::Vec;

#[cfg(feature = "std")]
use alloc::vec;
#[cfg(feature = "std")]
use std::fs::File;

#[cfg(all(feature = "async", not(feature = "tokio-support")))]
use blocking::{unblock, Unblock};

#[cfg(feature = "async")]
use futures_lite::{AsyncRead, AsyncReadExt};

#[cfg(feature = "tokio-support")]
use tokio_util::compat::TokioAsyncReadCompatExt as _;

/// Information needed to authorize a user to use an X11 connection.
#[derive(Default, Debug)]
pub struct AuthInfo {
    pub name: Vec<u8>,
    pub data: Vec<u8>,

    pub family: u16,
    pub address: Vec<u8>,
    pub number: Vec<u8>,
}

impl AuthInfo {
    /// Get an AuthInfo from the local Xauthority file.
    #[inline]
    pub fn get(family: u16, address: &[u8], display: u16) -> crate::Result<Option<AuthInfo>> {
        cfg_if::cfg_if! {
            if #[cfg(any(test, not(feature = "std")))] {
                Ok(None)
            } else {
                let f = match file::xauth_file().map(File::open) {
                    Some(Ok(f)) => f,
                    Some(Err(e)) => return Err(e.into()),
                    None => return Ok(None),
                };

                let a = get::get_auth(reader::auth_info_reader(f), family, address, display)?;
                Ok(a)
            }
        }
    }

    /// Get an AuthInfo from the local XAuthority file.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_async(
        family: u16,
        address: &[u8],
        display: u16,
    ) -> crate::Result<Option<AuthInfo>> {
        if cfg!(test) {
            return Ok(None);
        }

        #[inline]
        async fn inner<R: AsyncRead + Unpin>(
            r: R,
            family: u16,
            address: &[u8],
            display: u16,
        ) -> crate::Result<Option<AuthInfo>> {
            let a =
                get::get_auth_async(reader::auth_info_reader_async(r), family, address, display)
                    .await?;
            Ok(a)
        }

        let name = match file::xauth_file() {
            Some(name) => name,
            None => return Ok(None),
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "tokio-support")] {
                let file = tokio::fs::File::open(&name).await?.compat();
                inner(file, family, address, display).await
            } else {
                let file = unblock(move || File::open(&name)).await?;
                let file = Unblock::new(file);
                inner(file, family, address, display).await
            }
        }
    }
}
