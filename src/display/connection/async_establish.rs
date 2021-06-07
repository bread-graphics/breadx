// MIT/Apache2 License

use super::{create_setup, AsyncConnection, AsyncConnectionExt};
use crate::{
    auth_info::AuthInfo,
    auto::{xproto::Setup, AsByteSequence},
    xid::XidGenerator,
    Fd,
};
use alloc::{boxed::Box, vec, vec::Vec};
use core::{
    future::Future,
    iter, mem,
    pin::Pin,
    task::{Context, Poll},
};
use tinyvec::TinyVec;

/// Future returned by `establish_async`.
#[must_use = "futures do nothing unless polled or .awaited"]
pub enum EstablishConnectionFuture<'a, C: ?Sized> {
    /// We are currently trying to resolve for the `AuthInfo`.
    #[doc(hidden)]
    ResolvingAuthInfo {
        conn: &'a mut C,
        auth_info_get_future: Pin<Box<dyn Future<Output = AuthInfo> + Send + Sync>>,
    },
    /// We are currently sending packets for the setup request.
    #[doc(hidden)]
    SendSetupRequest {
        conn: &'a mut C,
        bytes: TinyVec<[u8; 32]>,
    },
    /// We are currently reading the setup.
    #[doc(hidden)]
    ReadSetupBytes {
        conn: &'a mut C,
        buffer: TinyVec<[u8; 32]>,
        cursor: usize,
        initial_eight_bytes: bool,
    },
    /// Completed.
    #[doc(hidden)]
    Complete,
}

impl<'a, C: ?Sized> EstablishConnectionFuture<'a, C> {
    #[inline]
    pub(crate) fn run(conn: &'a mut C, auth_info: Option<AuthInfo>) -> Self {
        match auth_info {
            None => EstablishConnectionFuture::ResolvingAuthInfo {
                conn,
                auth_info_get_future: Box::pin(AuthInfo::get_async()),
            },
            Some(auth) => EstablishConnectionFuture::SendSetupRequest {
                conn,
                bytes: setup_bytes(auth),
            },
        }
    }
}

#[inline]
fn setup_bytes(auth_info: AuthInfo) -> TinyVec<[u8; 32]> {
    let setup = create_setup(auth_info);
    let mut bytes: TinyVec<[u8; 32]> = iter::repeat(0).take(setup.size()).collect();
    let len = setup.as_bytes(&mut bytes);
    bytes.truncate(len);
    bytes
}

impl<'a, C: AsyncConnection + Unpin + ?Sized> Future for EstablishConnectionFuture<'a, C> {
    type Output = crate::Result<(Setup, XidGenerator)>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match mem::replace(&mut *self, EstablishConnectionFuture::Complete) {
                EstablishConnectionFuture::Complete => {
                    panic!("Attempted to poll future after completion")
                }
                EstablishConnectionFuture::ResolvingAuthInfo {
                    conn,
                    mut auth_info_get_future,
                } => match auth_info_get_future.as_mut().poll(cx) {
                    Poll::Pending => {
                        *self = EstablishConnectionFuture::ResolvingAuthInfo {
                            conn,
                            auth_info_get_future,
                        };
                        return Poll::Pending;
                    }
                    Poll::Ready(auth_info) => {
                        *self = EstablishConnectionFuture::SendSetupRequest {
                            conn,
                            bytes: setup_bytes(auth_info),
                        };
                    }
                },
                EstablishConnectionFuture::SendSetupRequest { conn, mut bytes } => {
                    let mut bytes_sent = 0;
                    let mut _fds = vec![];
                    let res = conn.poll_send_packet(&mut bytes, &mut _fds, cx, &mut bytes_sent);
                    bytes = bytes.split_off(bytes_sent);
                    match res {
                        Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                        Poll::Pending => {
                            *self = EstablishConnectionFuture::SendSetupRequest { conn, bytes };
                            return Poll::Pending;
                        }
                        Poll::Ready(Ok(())) => {
                            bytes.truncate(8);
                            *self = EstablishConnectionFuture::ReadSetupBytes {
                                conn,
                                buffer: iter::repeat(0).take(8).collect(),
                                cursor: 0,
                                initial_eight_bytes: true,
                            };
                        }
                    }
                }
                EstablishConnectionFuture::ReadSetupBytes {
                    conn,
                    mut buffer,
                    mut cursor,
                    initial_eight_bytes,
                } => {
                    let mut _fds = vec![];
                    let mut bytes_read = 0;
                    let res = conn.poll_read_packet(
                        &mut buffer[cursor..],
                        &mut _fds,
                        cx,
                        &mut bytes_read,
                    );
                    cursor += bytes_read;

                    match res {
                        Poll::Pending => {
                            *self = EstablishConnectionFuture::ReadSetupBytes {
                                conn,
                                buffer,
                                cursor,
                                initial_eight_bytes,
                            };
                            return Poll::Pending;
                        }
                        Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                        Poll::Ready(Ok(())) => {
                            if initial_eight_bytes {
                                // figure out whether or not it succeeded
                                match buffer[0] {
                                    0 => {
                                        return Poll::Ready(Err(crate::BreadError::FailedToConnect))
                                    }
                                    2 => {
                                        return Poll::Ready(Err(
                                            crate::BreadError::FailedToAuthorize,
                                        ))
                                    }
                                    _ => (),
                                }

                                // read in the rest of the setup
                                let length =
                                    u16::from_ne_bytes([buffer[6], buffer[7]]) as usize * 4;
                                buffer.extend(iter::repeat(0).take(length));

                                *self = EstablishConnectionFuture::ReadSetupBytes {
                                    conn,
                                    buffer,
                                    cursor,
                                    initial_eight_bytes: false,
                                };
                            } else {
                                let (setup, _) = match Setup::from_bytes(&buffer) {
                                    Some(s) => s,
                                    None => {
                                        return Poll::Ready(Err(crate::BreadError::BadObjectRead(
                                            Some("Setup"),
                                        )))
                                    }
                                };
                                let xid = XidGenerator::new(
                                    setup.resource_id_base,
                                    setup.resource_id_mask,
                                );
                                return Poll::Ready(Ok((setup, xid)));
                            }
                        }
                    }
                }
            }
        }
    }
}
