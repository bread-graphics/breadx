// MIT/Apache2 License

#![cfg(feature = "async")]

use super::NameConnection;
use crate::{Error, Result};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::{future::Future, mem, pin::Pin, task::Context};
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::mpsc;

use x11rb_protocol::parse_display::ConnectAddress;

/// Connect to the server in a non-blocking manner.
pub(crate) async fn nb_connect(ca: ConnectAddress<'_>) -> Result<NameConnection> {
    let mut error: Option<Error> = None;

    for sock_addr in sock_addrs(ca).await {
        let SocketDetails {
            addr,
            domain,
            protocol,
        } = sock_addr;

        match connect(addr, domain, protocol) {
            Ok(socket) => {
                #[cfg(unix)]
                {
                    if protocol.is_none() {
                        return Ok(NameConnection::from_unix_stream(socket));
                    }
                }

                return Ok(NameConnection::from_tcp_stream(socket.into()));
            }
            Err(e) => {
                error = Some(Error::io(e));
            }
        }
    }

    Err(error.unwrap_or_else(|| Error::couldnt_parse_display()))
}

/// Get the socket addresses we need to connect to.
async fn sock_addrs(ca: ConnectAddress<'_>) -> Result<Vec<SocketDetails>> {
    match ca {
        ConnectAddress::Hostname(hostname, port) => {
            // collect the potential addresses
            Ok(tcp_ip_addr(hostname, port)
                .await?
                .into_iter()
                .map(|addr| {
                    let domain = Domain::for_address(addr);

                    SocketDetails {
                        addr: domain.into(),
                        domain,
                        protocol: Some(Protocol::Tcp),
                    }
                })
                .collect())
        }
        ConnectAddress::Socket(path) => {
            cfg_if::cfg_if! {
                if #[cfg(unix)] {
                    // begin a Unix domain socket connection to the path
                    Ok(SocketDetails {
                        addr: SockAddr::unix(path).map_err(Error::io)?,
                        domain: Domain::UNIX,
                        protocol: None,
                    })
                } else {
                    Err(Error::unsupported(Unsupported::Socket))
                }
            }
        }
    }
}

fn connect(addr: SockAddr, domain: Domain, protocol: Option<Protocol>) -> Result<Socket> {
    let sock_type = Type::STREAM;
    #[cfg(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "illumos",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    // If we can, set nonblocking at socket creation for unix
    let sock_type = sock_type.nonblocking();
    // This automatically handles cloexec on unix, no_inherit on windows and nosigpipe on macos
    let socket = Socket::new(domain, sock_type, protocol).map_err(Error::io)?;
    #[cfg(not(any(
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "fuchsia",
        target_os = "illumos",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd"
    )))]
    // If the current platform doesn't support nonblocking at creation, enable it after creation
    socket.set_nonblocking(true).map_err(Error::io)?;
    match socket.connect(&addr) {
        Ok(_) => {}
        #[cfg(unix)]
        Err(err) if err.raw_os_error() == Some(libc::EINPROGRESS) => {}
        Err(err) if err.kind() == io::ErrorKind::WouldBlock => {}
        Err(err) => return Err(Error::io(err)),
    }
    Ok(socket)
}

/// Get the IP address for a TCP connection.
async fn tcp_ip_addr(hostname: &str, port: u16) -> Result<Vec<SocketAddr>> {
    // try to fast path our way out of this; parse the hostname as an IP address
    if let Ok(ip) = hostname.parse::<Ipv4Addr>() {
        return Ok(vec![SocketAddr::new(IpAddr::V4(ip), port)]);
    }

    if let Ok(ip) = hostname.parse::<Ipv6Addr>() {
        return Ok(vec![SocketAddr::new(IpAddr::V6(ip), port)]);
    }

    // slow path: open up a dedicated threadpool and resolve the hostname
    // using DNS
    //
    // X11 isn't really connected to much over hostname anymore, so we can
    // consider this the cold path
    let socket_addr = (String::from(hostname), port);
    Unblock::new(move || std::net::ToSocketAddrs::to_socket_addrs(socket_addr)).await
}

/// A future that spawns a dedicated thread to run a blocking iterator.
///
/// Similar to tokio::spawn_blocking or blocking::unblock, but this is
/// runtime independent and probably much slower.
enum Unblock<I: Iterator, F> {
    Unspawned {
        iterator: I,
    },
    Spawned {
        pool: JoinHandle<()>,
        output: mpsc::Receiver<Result<I::Item>>,
        waker: mpsc::Sender<Waker>,
        collection: Vec<I::Item>,
    },
    Hole,
}

impl<I: Iterator + 'static, F: FnOnce() -> Result<I>> Unblock<I>
where
    I::Item: 'static,
{
    fn new(iterator: F) -> Self {
        Unblock::Unspawned { iterator }
    }

    fn spawn(&mut self) {
        let (tx, rx) = mpsc::channel();
        let (waker_tx, waker_rx) = mpsc::channel();

        // get the iterator out
        let this = mem::replace(self, Unblock::Hole);

        let iterator = match this {
            Unblock::Unspawned { iterator } => iterator,
            _ => unreachable!(),
        };

        // spawn the thread
        let handle = thread::Builder::new()
            .name("breadx-dns-thread".into())
            .spawn(move || {
                // generate the iterator
                let iterator = match iterator() {
                    Ok(iterator) => iterator,
                    Err(e) => {
                        // notify and die
                        let _ = tx.send(Err(e));
                        return;
                    }
                };

                // run the iterator
                for item in iterator {
                    if tx.send(Ok(item)).is_err() {
                        break;
                    }
                }

                // drop tx to signal the end
                mem::drop(tx);

                // wake the future
                while let Ok(waker) = waker_rx.recv() {
                    waker.wake();
                }
            })
            .expect("failed to spawn unblock thread");

        *self = Unblock::Spawned {
            pool: handle,
            output: rx,
            waker: waker_tx,
            collection: vec![],
        };
    }
}

impl<I: Iterator + 'static, F: FnOnce() -> I> Future for Unblock
where
    I::Item: 'static,
{
    type Output = Result<Vec<I::Item>>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        // if we haven't spawned yet, do so
        if let Unblock::Unspawned { .. } = self.get_mut() {
            self.spawn();
        }

        // if we're still waiting, wait
        if let Unblock::Spawned { .. } = self.get_mut() {
            // try to receive an item from the channel
            loop {
                match self.get_mut().output.try_recv() {
                    Ok(Ok(item)) => {
                        self.get_mut().collection.push(item);
                    }
                    Ok(Err(e)) => {
                        // iterator errored out, die
                        return Poll::Ready(Err(e));
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        // if we're still waiting, wait
                        let _ = self.get_mut().waker.send(ctx.waker().clone());
                        return Poll::Pending;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        // it's over
                        return Poll::Ready(Ok(mem::take(&mut self.get_mut().collection)));
                    }
                }
            }
        } else {
            unreachable!();
        }
    }
}

struct SocketDetails {
    addr: SockAddr,
    domain: Domain,
    protocol: Option<Protocol>,
}
