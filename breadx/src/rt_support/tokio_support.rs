// MIT/Apache2 License

#![cfg(all(feature = "tokio-support", unix))]

// TODO: support async windows sockets if/when they come out in tokio

use crate::{
    connection::Connection,
    display::{
        AsyncDisplay, AsyncStatus, BasicDisplay, CanBeAsyncDisplay, DisplayBase, DisplayConnection,
        Interest, RawReply, RawRequest,
    },
    Error, NameConnection, Result,
};
use alloc::{string::ToString, vec, vec::Vec};
use core::{
    future::Future,
    task::{Context, Poll},
};
use std::os::unix::io::AsRawFd;
use tokio::io::unix::{AsyncFd, AsyncFdReadyMutGuard};
use tracing::Instrument;
use x11rb_protocol::{
    connect::Connect,
    parse_display,
    protocol::{xproto::Setup, Event},
    xauth,
};

/* Main trait impls */

impl<D: AsRawFd + CanBeAsyncDisplay> AsyncDisplay for AsyncFd<D> {
    fn poll_for_interest(
        &mut self,
        interest: Interest,
        callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<()>,
        ctx: &mut Context<'_>,
    ) -> Poll<Result<()>> {
        let span = tracing::trace_span!(
            "tokio_support::poll_for_interest",
            interest = ?interest
        );
        let _enter = span.enter();

        tracing::debug!("polling for interest in {:?}", interest);

        // determine which interest we are polling on
        let guard = match interest {
            Interest::Readable => self.poll_read_ready_mut(ctx),
            Interest::Writable => self.poll_write_ready_mut(ctx),
        };

        tracing::trace!(is_ready = guard.is_ready(), "polled for guard");

        let mut guard = match guard {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(Error::io(e))),
            Poll::Ready(Ok(guard)) => guard,
        };

        // try for I/O on the socket
        let mut result: Result<()> = Ok(());

        let io_result = guard
            .try_io(|item| {
                // call the callback
                if let Err(e) = callback(item, ctx) {
                    // if it's an I/O error, bubble it up so tokio can deal with it
                    match e.into_io_error() {
                        Ok(io_error) => return Err(io_error),
                        Err(e) => {
                            result = Err(e);
                        }
                    }
                }

                Ok(())
            })
            .unwrap_or(Ok(()));

        Poll::Ready(io_result.map_err(Error::io).and(result))
    }
}

impl<'lt, D: AsRawFd + DisplayBase> AsyncDisplay for &'lt AsyncFd<D>
where
    &'lt D: CanBeAsyncDisplay,
{
    fn poll_for_interest(
        &mut self,
        interest: Interest,
        callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<()>,
        ctx: &mut Context<'_>,
    ) -> Poll<Result<()>> {
        // same as above but without the mut functions
        let guard = match interest {
            Interest::Readable => self.poll_read_ready(ctx),
            Interest::Writable => self.poll_write_ready(ctx),
        };

        let mut guard = match guard {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(Error::io(e))),
            Poll::Ready(Ok(guard)) => guard,
        };

        // try for I/O on the socket
        let mut result: Result<()> = Ok(());

        let io_result = guard
            .try_io(|mut guard| {
                // run the I/O function
                if let Err(e) = callback(&mut guard, ctx) {
                    // if it's an I/O error, bubble it up so tokio can deal with it
                    match e.into_io_error() {
                        Ok(io_error) => return Err(io_error),
                        Err(e) => {
                            result = Err(e);
                        }
                    }
                }

                Ok(())
            })
            .unwrap_or(Ok(()));

        Poll::Ready(io_result.map_err(Error::io).and(result))
    }
}

/* Connection */

/// # Panics
///
/// Panics if `tokio` fails to join the blocking task used to compute authorization details.
pub fn connect(name: Option<&str>) -> impl Future<Output = Result<AsyncFd<DisplayConnection>>> {
    let name = name.map(ToString::to_string);
    async move {
        // try to create a name connection
        let dpy = parse_display::parse_display(name.as_deref())
            .ok_or_else(|| Error::couldnt_parse_display(name.is_none()))?;

        let screen = dpy.screen;
        let display_num = dpy.display;
        let conn =
            NameConnection::from_parsed_display_async(&dpy, name.is_none(), |name| async move {
                // poll the display until it is writable
                let registered = AsyncFd::new(name).map_err(Error::io)?;
                let mut guard = registered.writable().await.map_err(Error::io)?;
                guard.retain_ready();
                let name = registered.into_inner();

                // check socket error
                if let Some(err) = name.take_error() {
                    Err(err)
                } else {
                    Ok(name)
                }
            })
            .await?;

        // find xauth
        let (family, address) = conn.get_address()?;

        // xauth uses file I/O, use blocking
        let (name, data) = tokio::task::spawn_blocking(move || {
            match xauth::get_auth(family, &address, display_num).map_err(Error::io) {
                Err(e) => Err(e),
                Ok(Some(auth)) => Ok(auth),
                Ok(None) => {
                    tracing::warn!("No Xauth found for display {}", display_num);

                    Ok((vec![], vec![]))
                }
            }
        })
        .await
        .unwrap()?;

        // run the connector code
        establish_connect(conn.into(), screen as usize, name, data).await
    }
}

pub fn establish_connect<Conn: AsRawFd + Connection>(
    conn: Conn,
    default_screen: usize,
    auth_name: Vec<u8>,
    auth_info: Vec<u8>,
) -> impl Future<Output = Result<AsyncFd<BasicDisplay<Conn>>>> {
    let span = tracing::info_span!("establish_connect");

    async move {
        // use the connect() struct to get a connection
        let (mut connect, setup_request) = Connect::with_authorization(auth_name, auth_info);
        let mut registered = AsyncFd::new(conn).map_err(Error::io)?;

        // use the system
        let mut nwritten = 0;
        while nwritten < setup_request.len() {
            // wait for it to be available
            let guard = registered.writable_mut().await.map_err(Error::io)?;

            // write to it
            let n = try_io(
                guard,
                |conn: &mut Conn| conn.send_slice(&setup_request[nwritten..]),
                || 0,
            )?;

            nwritten += n;
        }

        // flush the reply
        loop {
            let guard = registered.writable_mut().await.map_err(Error::io)?;

            let finished = try_io(
                guard,
                |conn: &mut Conn| {
                    conn.flush()?;
                    Ok(true)
                },
                || false,
            )?;
            if finished {
                break;
            }
        }

        // read until we're finished
        loop {
            let guard = registered.readable_mut().await.map_err(Error::io)?;
            let adv = try_io(
                guard,
                |conn: &mut Conn| conn.recv_slice(connect.buffer()),
                || 0,
            )?;

            if connect.advance(adv) {
                break;
            }
        }

        let setup = connect.into_setup().map_err(Error::make_connect_error)?;
        let dpy = BasicDisplay::with_connection(registered.into_inner(), setup, default_screen)?;
        AsyncFd::new(dpy).map_err(Error::io)
    }
    .instrument(span)
}

fn try_io<C: AsRawFd + Connection, R>(
    mut guard: AsyncFdReadyMutGuard<'_, C>,
    f: impl FnOnce(&mut C) -> Result<R>,
    blocked: impl FnOnce() -> R,
) -> Result<R> {
    let res = guard
        .try_io(move |registered| {
            match f(registered.get_mut()) {
                Ok(r) => Ok(Ok(r)),
                Err(e) => {
                    // transform it into an I/O error if possible
                    match e.into_io_error() {
                        Ok(ioe) => Err(ioe),
                        Err(e) => Ok(Err(e)),
                    }
                }
            }
        })
        .unwrap_or_else(move |_| Ok(Ok(blocked())))
        .map_err(Error::io)??;

    Ok(res)
}

/* Trait forwarding for AsyncFd */

impl<D: AsRawFd + DisplayBase> DisplayBase for AsyncFd<D> {
    fn setup(&self) -> &Setup {
        self.get_ref().setup()
    }

    fn default_screen_index(&self) -> usize {
        self.get_ref().default_screen_index()
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.get_mut().poll_for_event()
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.get_mut().poll_for_reply_raw(seq)
    }
}

impl<'lt, D: AsRawFd + DisplayBase> DisplayBase for &'lt AsyncFd<D>
where
    &'lt D: DisplayBase,
{
    fn setup(&self) -> &Setup {
        self.get_ref().setup()
    }

    fn default_screen_index(&self) -> usize {
        self.get_ref().default_screen_index()
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.get_ref().poll_for_event()
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.get_ref().poll_for_reply_raw(seq)
    }
}

impl<D: AsRawFd + CanBeAsyncDisplay> CanBeAsyncDisplay for AsyncFd<D> {
    fn format_request(
        &mut self,
        req: &mut RawRequest<'_, '_>,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<u64>> {
        self.get_mut().format_request(req, ctx)
    }

    fn try_send_request_raw(
        &mut self,
        req: &mut RawRequest<'_, '_>,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<()>> {
        self.get_mut().try_send_request_raw(req, ctx)
    }

    fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
        self.get_mut().try_wait_for_event(ctx)
    }

    fn try_wait_for_reply_raw(
        &mut self,
        seq: u64,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<RawReply>> {
        self.get_mut().try_wait_for_reply_raw(seq, ctx)
    }

    fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
        self.get_mut().try_flush(ctx)
    }

    fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
        self.get_mut().try_maximum_request_length(ctx)
    }

    fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
        self.get_mut().try_generate_xid(ctx)
    }
}

impl<'lt, D: AsRawFd + DisplayBase> CanBeAsyncDisplay for &'lt AsyncFd<D>
where
    &'lt D: CanBeAsyncDisplay,
{
    fn format_request(
        &mut self,
        req: &mut RawRequest<'_, '_>,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<u64>> {
        self.get_ref().format_request(req, ctx)
    }

    fn try_send_request_raw(
        &mut self,
        req: &mut RawRequest<'_, '_>,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<()>> {
        self.get_ref().try_send_request_raw(req, ctx)
    }

    fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
        self.get_ref().try_wait_for_event(ctx)
    }

    fn try_wait_for_reply_raw(
        &mut self,
        seq: u64,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<RawReply>> {
        self.get_ref().try_wait_for_reply_raw(seq, ctx)
    }

    fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
        self.get_ref().try_flush(ctx)
    }

    fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
        self.get_ref().try_generate_xid(ctx)
    }

    fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
        self.get_ref().try_maximum_request_length(ctx)
    }
}
