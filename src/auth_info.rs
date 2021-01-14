// MIT/Apache2 License

use alloc::{string::String, vec::Vec};

#[cfg(feature = "std")]
use alloc::vec;
#[cfg(feature = "std")]
use std::{env, fs::File, io::Read};

#[cfg(feature = "async")]
use blocking::{unblock, Unblock};
#[cfg(feature = "async")]
use futures_lite::{AsyncRead, AsyncReadExt};

/// Information needed to authorize a user to use an X11 connection.
#[derive(Default, Debug)]
pub struct AuthInfo {
    pub name: String,
    pub data: Vec<u8>,

    pub family: u16,
    pub address: Vec<u8>,
    pub number: Vec<u8>,
}

/// Helper: from a set of bytes, deserialize a "counted string"
#[cfg(feature = "std")]
#[inline]
fn counted_string(bytes: &mut &[u8]) -> Option<Vec<u8>> {
    if bytes.len() < 2 {
        log::error!("Auth did not contain length bytes");
        return None;
    }

    // read in the length
    let length: [u8; 2] = [bytes[0], bytes[1]];
    let length = u16::from_be_bytes(length) as usize;

    if bytes.len() < 2 + length {
        log::error!("Auth did not contain string bytes");
        return None;
    }

    let res = (&bytes[2..length + 2]).to_vec();
    *bytes = &bytes[length + 2..];
    Some(res)
}

#[cfg(feature = "std")]
impl AuthInfo {
    /// Reads the auth info in from a buffer format.
    #[inline]
    fn from_buffer(s: &mut &[u8]) -> Option<Self> {
        if s.len() < 2 {
            log::error!("Auth did not contain family bytes");
            return None;
        }

        // read in the family
        let family: [u8; 2] = [s[0], s[1]];
        let family = u16::from_be_bytes(family);

        let mut cursor = &s[2..];
        let address = counted_string(&mut cursor)?;
        let number = counted_string(&mut cursor)?;
        let name = match String::from_utf8(counted_string(&mut cursor)?) {
            Ok(name) => name,
            Err(e) => {
                // try to replace non-utf8 bytes with utf-8 bytes
                log::warn!("Name was not valid UTF-8, doing substitution.");
                let mut name = e.into_bytes();
                name.retain(|b| *b < 128);
                String::from_utf8(name).unwrap()
            }
        };
        let data = counted_string(&mut cursor)?;

        *s = &s[10 + address.len() + number.len() + name.len() + data.len()..];

        Some(AuthInfo {
            family,
            address,
            number,
            name,
            data,
        })
    }

    /// Reads in several authorization informations from a buffer.
    #[inline]
    fn many_from_buffer(mut s: &[u8]) -> Option<Vec<Self>> {
        let mut res = vec![];
        while !s.is_empty() {
            res.push(Self::from_buffer(&mut s)?);
        }
        Some(res)
    }

    /// Reads in the auth info from the given reading stream.
    #[inline]
    #[must_use]
    pub fn from_stream<R: Read>(reader: &mut R) -> Option<Vec<Self>> {
        let mut buffer = Vec::with_capacity(128);
        let _ = reader.read_to_end(&mut buffer).ok()?;

        Self::many_from_buffer(&buffer)
    }

    /// Reads in the auth info from the given reading stream, async redox.
    #[cfg(feature = "async")]
    #[inline]
    #[must_use]
    pub async fn from_stream_async<R: AsyncRead + Unpin>(reader: &mut R) -> Option<Vec<Self>> {
        let mut buffer = Vec::with_capacity(128);
        let _ = reader.read_to_end(&mut buffer).await.ok()?;

        Self::many_from_buffer(&buffer)
    }

    /// Reads in the auth info from the file specified by the `XAUTHORITY` environment variable.
    #[inline]
    #[must_use]
    pub fn from_xauthority() -> Option<Vec<Self>> {
        let fname = env::var_os("XAUTHORITY")?;
        let mut file = File::open(&fname).ok()?;
        Self::from_stream(&mut file)
    }

    /// Reads in the auth info from the file specified by the XAuthority variable, async redox.
    #[cfg(feature = "async")]
    #[inline]
    #[must_use]
    pub async fn from_xauthority_async() -> Option<Vec<Self>> {
        let fname = env::var_os("XAUTHORITY")?;
        let file = unblock(move || File::open(&fname)).await.ok()?;
        let mut file = Unblock::new(file);
        Self::from_stream_async(&mut file).await
    }

    /// Helper function to "get" an authorization info or return the default.
    /// TODO: match up display ID
    #[inline]
    pub(crate) fn get() -> Self {
        if let Some(mut v) = Self::from_xauthority() {
            if v.is_empty() {
                Default::default()
            } else {
                v.remove(0)
            }
        } else {
            log::error!("Failed to get AuthInfo from XAUTHORITY, using empty auth info");
            Default::default()
        }
    }

    #[cfg(feature = "async")]
    #[inline]
    pub(crate) async fn get_async() -> Self {
        match Self::from_xauthority_async().await {
            Some(mut v) => {
                if v.is_empty() {
                    Default::default()
                } else {
                    v.remove(0)
                }
            }
            None => Default::default(),
        }
    }
}

#[cfg(not(feature = "std"))]
impl AuthInfo {
    /// Helper function to "get" an authorization info or return the default.
    #[inline]
    pub(crate) fn get() -> Self {
        Default::default()
    }
}
