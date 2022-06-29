// MIT/Apache2 License

#![cfg(feature = "async-std-support")]

cfg_std_unix! {
    use std::os::unix::io::AsRawFd;
}

cfg_std_windows! {
    use std::os::windows::io::AsRawSocket;
}

use core::task::{Context, Poll};

use crate::{
    connection::Connection,
    display::{
        AsyncDisplay, AsyncStatus, BasicDisplay, CanBeAsyncDisplay, DisplayBase, DisplayConnection,
        Interest, RawReply, RawRequest,
    },
    Error, NameConnection, Result,
};
use alloc::{string::ToString, sync::Arc, vec, vec::Vec};
use async_io::Async;
use core::future::Future;
use tracing::Instrument;
use x11rb_protocol::{
    connect::Connect,
    parse_display,
    protocol::{xproto::Setup, Event},
    xauth,
};

// create a "Source" trait that aliases to AsRawFd
// or AsRawSocket, depending on what's good

cfg_std_unix! {
    #[doc(hidden)]
    pub trait Source: AsRawFd {}
    impl<T: AsRawFd> Source for T {}
}

cfg_std_windows! {
    #[doc(hidden)]
    pub trait Source: AsRawSocket {}
    impl<T: AsRawSocket> Source for T {}
}

// impl trait on top of Async

impl<D: CanBeAsyncDisplay + Source> AsyncDisplay for Async<D> {
    fn poll_for_interest(
        &mut self,
        interest: Interest,
        callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<()>,
        ctx: &mut Context<'_>,
    ) -> Poll<Result<()>> {
        let span = tracing::trace_span!(
            "async_std_support::poll_for_interest",
            interest = ?interest
        );
        let _enter = span.enter();

        match poll_ready(self, interest, ctx) {
            Poll::Ready(Ok(())) => {}
            poll => return poll,
        }

        // try for I/O on the socket
        match callback(self, ctx) {
            Err(e) if e.would_block() => {
                // indicate that we should poll again
                ctx.waker().wake_by_ref();
                Poll::Pending
            }
            poll => Poll::Ready(poll),
        }
    }
}

impl<'lt, D: DisplayBase + Source> AsyncDisplay for &'lt Async<D>
where
    &'lt D: CanBeAsyncDisplay,
{
    fn poll_for_interest(
        &mut self,
        interest: Interest,
        callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<()>,
        ctx: &mut Context<'_>,
    ) -> Poll<Result<()>> {
        let span = tracing::trace_span!(
            "async_std_support::poll_for_interest",
            interest = ?interest
        );
        let _enter = span.enter();

        match poll_ready(self, interest, ctx) {
            Poll::Ready(Ok(())) => {}
            poll => return poll,
        }

        // try for I/O on the socket
        match callback(self, ctx) {
            Err(e) if e.would_block() => {
                ctx.waker().wake_by_ref();
                Poll::Pending
            }
            poll => Poll::Ready(poll),
        }
    }
}

fn poll_ready<D>(a: &Async<D>, interest: Interest, ctx: &mut Context<'_>) -> Poll<Result<()>> {
    tracing::trace!("polling for interest in {:?}", interest);

    // poll for the interest
    let res = match interest {
        Interest::Readable => a.poll_readable(ctx),
        Interest::Writable => a.poll_writable(ctx),
    };

    tracing::trace!(is_ready = res.is_ready(), "polled for readiness");

    res.map_err(Error::io)
}

// connection forming

pub fn connect(name: Option<&str>) -> impl Future<Output = Result<Async<DisplayConnection>>> {
    let name = name.map(ToString::to_string);
    async move {
        // create a name connection
        let dpy = parse_display::parse_display(name.as_deref())
            .ok_or_else(|| Error::couldnt_parse_display(name.is_none()))?;

        let screen = dpy.screen;
        let display_num = dpy.display;
        let conn =
            NameConnection::from_parsed_display_async(&dpy, name.is_none(), |name| async move {
                // poll the display until it is writable
                let registered = Async::new(name).map_err(Error::io)?;
                registered.writable().await.map_err(Error::io)?;
                let name = registered.into_inner().map_err(Error::io)?;

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
        let (name, data) = blocking::unblock(move || {
            match xauth::get_auth(family, &address, display_num).map_err(Error::io) {
                Err(e) => Err(e),
                Ok(Some(auth)) => Ok(auth),
                Ok(None) => {
                    tracing::warn!("No Xauth found for display {}", display_num);

                    Ok((vec![], vec![]))
                }
            }
        })
        .await?;

        // run the connector code
        establish_connect(conn.into(), screen as usize, name, data).await
    }
}

pub fn establish_connect<Conn: Source + Connection>(
    conn: Conn,
    default_screen: usize,
    auth_name: Vec<u8>,
    auth_data: Vec<u8>,
) -> impl Future<Output = Result<Async<BasicDisplay<Conn>>>> {
    let span = tracing::info_span!("establish_connect");

    async move {
        // use connect struct for establishing a connection
        let (mut connect, setup_request) = Connect::with_authorization(auth_name, auth_data);
        let mut registered = Async::new(conn).map_err(Error::io)?;

        // write as much as we can
        let mut written = 0;
        while written < setup_request.len() {
            write_with_mut(&mut registered, |conn| {
                let n = conn.send_slice(&setup_request[written..])?;
                written += n;
                Ok(())
            })
            .await?;
        }

        // flush the request
        write_with_mut(&mut registered, Connection::flush).await?;

        // read until we're finished
        loop {
            let adv =
                read_with_mut(&mut registered, |conn| conn.recv_slice(connect.buffer())).await?;

            if connect.advance(adv) {
                break;
            }
        }

        // we're finished
        let setup = connect.into_setup().map_err(Error::make_connect_error)?;
        let dpy = BasicDisplay::with_connection(
            registered.into_inner().map_err(Error::io)?,
            setup,
            default_screen,
        )?;
        Async::new(dpy).map_err(Error::io)
    }
    .instrument(span)
}

async fn write_with_mut<D, R: Default>(
    a: &mut Async<D>,
    mut f: impl FnMut(&mut D) -> Result<R>,
) -> Result<R> {
    let mut res: Result<()> = Ok(());
    let io_res = a
        .write_with_mut(|conn| match f(conn) {
            Ok(r) => Ok(r),
            Err(e) => match e.into_io_error() {
                Ok(e) => Err(e),
                Err(e) => {
                    res = Err(e);
                    Ok(Default::default())
                }
            },
        })
        .await;

    res.and(io_res.map_err(Error::io))
}

async fn read_with_mut<D, R: Default>(
    a: &mut Async<D>,
    mut f: impl FnMut(&mut D) -> Result<R>,
) -> Result<R> {
    let mut res: Result<()> = Ok(());
    let io_res = a
        .read_with_mut(|conn| match f(conn) {
            Ok(r) => Ok(r),
            Err(e) => match e.into_io_error() {
                Ok(e) => Err(e),
                Err(e) => {
                    res = Err(e);
                    Ok(Default::default())
                }
            },
        })
        .await;

    res.and(io_res.map_err(Error::io))
}

// trait forwarding

impl<D: DisplayBase> DisplayBase for Async<D> {
    fn setup(&self) -> &Arc<Setup> {
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

impl<'lt, D: DisplayBase> DisplayBase for &'lt Async<D>
where
    &'lt D: DisplayBase,
{
    fn setup(&self) -> &Arc<Setup> {
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

impl<D: CanBeAsyncDisplay> CanBeAsyncDisplay for Async<D> {
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

    fn try_wait_for_reply_raw(
        &mut self,
        seq: u64,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<RawReply>> {
        self.get_mut().try_wait_for_reply_raw(seq, ctx)
    }

    fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
        self.get_mut().try_wait_for_event(ctx)
    }

    fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
        self.get_mut().try_flush(ctx)
    }

    fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
        self.get_mut().try_generate_xid(ctx)
    }

    fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
        self.get_mut().try_maximum_request_length(ctx)
    }

    fn try_check_for_error(&mut self,seq:u64,ctx: &mut Context< '_>) -> Result<AsyncStatus<()>> {
        self.get_mut().try_check_for_error(seq,ctx)
    }
}

impl<'lt, D: DisplayBase> CanBeAsyncDisplay for &'lt Async<D>
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

    fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
        self.get_ref().try_maximum_request_length(ctx)
    }

    fn try_wait_for_reply_raw(
        &mut self,
        seq: u64,
        ctx: &mut Context<'_>,
    ) -> Result<AsyncStatus<RawReply>> {
        self.get_ref().try_wait_for_reply_raw(seq, ctx)
    }

    fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
        self.get_ref().try_generate_xid(ctx)
    }

    fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
        self.get_ref().try_flush(ctx)
    }

    fn try_check_for_error(&mut self,seq:u64,ctx: &mut Context< '_>) -> Result<AsyncStatus<()>> {
        self.get_ref().try_check_for_error(seq,ctx)
    }
}
