// MIT/Apache2 License

use super::AuthInfo;
use std::{vec, vec::Vec, io::{self, Read, Seek}, boxed::Box, pin::Pin};

#[cfg(feature = "async")]
use futures_lite::{
    io as aio,
    stream::{self, Stream},
    AsyncRead, AsyncReadExt,
};

/// Create an iterator over the authorization information in an Authority file.
#[inline]
pub(crate) fn auth_info_reader<R: Read>(r: R) -> impl Iterator<Item = Result<AuthInfo, io::Error>> {
    AuthInfoReader {
        reader: io::BufReader::new(r),
    }
}

#[derive(Debug)]
struct AuthInfoReader<R> {
    reader: io::BufReader<R>,
}

impl<R: Read> Iterator for AuthInfoReader<R> {
    type Item = Result<AuthInfo, io::Error>;

    #[inline]
    fn next(&mut self) -> Option<Result<AuthInfo, io::Error>> {
        read_auth(&mut self.reader).transpose()
    }
}

#[inline]
fn read_auth<R: Read>(r: &mut R) -> Result<Option<AuthInfo>, io::Error> {
    let family = match read_short(r) {
        Ok(family) => family,
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    };

    let address = read_counted_string(r)?;
    let number = read_counted_string(r)?;
    let name = read_counted_string(r)?;
    let data = read_counted_string(r)?;

    Ok(Some(AuthInfo {
        family,
        data,
        number,
        name,
        address,
    }))
}

#[inline]
fn read_counted_string<R: Read>(r: &mut R) -> Result<Vec<u8>, io::Error> {
    let len = read_short(r)? as usize;
    let mut s: Vec<u8> = vec![0u8; len];
    r.read_exact(&mut s)?;
    Ok(s)
}

#[inline]
fn read_short<R: Read>(r: &mut R) -> Result<u16, io::Error> {
    let mut buffer = [0u8; 2];
    r.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}

// Async variant

/// Create an iterator over the authorization information in an Authority file, async redox.
#[cfg(feature = "async")]
#[inline]
pub(crate) fn auth_info_reader_async<'a, R: AsyncRead + Unpin + 'a>(
    r: R,
) -> Pin<Box<dyn Stream<Item = Result<AuthInfo, io::Error>> + 'a>> {
    let mut r = aio::BufReader::new(r);
    Box::pin(stream::try_unfold((), move |()| async move {
        read_auth_async(&mut r)
            .await
            .map(|opt| opt.map(|ai| (ai, ())))
    }))
}

#[cfg(feature = "async")]
#[inline]
async fn read_auth_async<R: AsyncRead + Unpin>(r: &mut R) -> Result<Option<AuthInfo>, io::Error> {
    let family = match read_short_async(r).await {
        Ok(family) => family,
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    };

    let address = read_counted_string_async(r).await?;
    let number = read_counted_string_async(r).await?;
    let name = read_counted_string_async(r).await?;
    let data = read_counted_string_async(r).await?;

    Ok(Some(AuthInfo {
        family,
        name,
        data,
        number,
        address,
    }))
}

#[cfg(feature = "async")]
#[inline]
async fn read_counted_string_async<R: AsyncRead + Unpin>(r: &mut R) -> Result<Vec<u8>, io::Error> {
    let len = read_short_async(r).await? as usize;
    let mut s = vec![0; len];
    r.read_exact(&mut s).await?;
    Ok(s)
}

#[cfg(feature = "async")]
#[inline]
async fn read_short_async<R: AsyncRead + Unpin>(r: &mut R) -> Result<u16, io::Error> {
    let mut buffer = [0u8; 2];
    r.read_exact(&mut buffer).await?;
    Ok(u16::from_be_bytes(buffer))
}
