// MIT/Apache2 License

#![cfg(feature = "async")]

use crate::{Error, NameConnection, Result, Unblock};
use alloc::{boxed::Box, string::ToString};
use core::{
    future::{self, Future},
    pin::Pin,
};
use futures_util::{
    stream::{self, StreamExt},
    Stream,
};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use x11rb_protocol::parse_display::{ConnectAddress, ParsedDisplay};

#[allow(unused_imports)]
use crate::Unsupported;

pub(crate) async fn nb_connect<Fut, R>(
    pd: &ParsedDisplay,
    is_env: bool,
    mut resolv: impl FnMut(NameConnection) -> Fut,
) -> Result<R>
where
    Fut: Future<Output = Result<R>>,
{
    // create a stream iterating over the possible connections
    let mut conns = stream::iter(pd.connect_instruction())
        .flat_map(instruction_into_socket)
        .map(|sd| sd.and_then(|(sd, mode)| sd.connect().map(move |sd| (sd, mode))))
        .map(|sd| {
            sd.map(|(socket, mode)| {
                // determine what mode to put them in
                if matches!(mode, SocketMode::Tcp) {
                    NameConnection::from_tcp_stream(socket.into())
                } else {
                    #[cfg(unix)]
                    {
                        NameConnection::from_unix_stream(socket.into())
                    }

                    #[cfg(not(unix))]
                    {
                        unreachable!()
                    }
                }
            })
        });

    // test them to see the first one that works
    // swap Ok to Err for try_fold
    let mut err: Option<Error> = None;

    while let Some(conn) = conns.next().await {
        match conn {
            Ok(conn) => match resolv(conn).await {
                Ok(conn) => return Ok(conn),
                Err(e) => {
                    err = Some(e);
                }
            },
            Err(e) => {
                err = Some(e);
            }
        }
    }

    Err(err.unwrap_or_else(|| Error::couldnt_parse_display(is_env)))
}

type SockAddrStream<'a> =
    Pin<Box<dyn Stream<Item = Result<(SocketDetails, SocketMode)>> + Send + 'a>>;

/// Convert a `ConnectInstruction` into a `Stream` iterating over the potential
/// socket details.
fn instruction_into_socket(ci: ConnectAddress<'_>) -> SockAddrStream<'_> {
    match ci {
        ConnectAddress::Hostname(hostname, port) => {
            // collect the potential addresses
            tcp_ip_addrs(hostname, port)
                .map(|addr| {
                    addr.map(|addr| {
                        let domain = Domain::for_address(addr);

                        (
                            SocketDetails {
                                addr: addr.into(),
                                domain,
                                protocol: Some(Protocol::TCP),
                            },
                            SocketMode::Tcp,
                        )
                    })
                })
                .boxed()
        }
        ConnectAddress::Socket(path) => {
            cfg_if::cfg_if! {
                if #[cfg(unix)] {
                    // unix socket for the path
                    let sock_details = SockAddr::unix(path).map_err(Error::io)
                        .map(|sock_addr| (SocketDetails {
                            addr: sock_addr,
                            domain: Domain::UNIX,
                            protocol: None,
                        }, SocketMode::Unix));

                    stream::once(future::ready(sock_details)).boxed()
                } else {
                    stream::once(future::ready(Err(
                        Err(Error::unsupported(crate::Unsupported::Socket))
                    ))).boxed()
                }
            }
        }
    }
}

fn tcp_ip_addrs(
    hostname: &str,
    port: u16,
) -> Pin<Box<dyn Stream<Item = Result<SocketAddr>> + Send + '_>> {
    // fast paths that don't involve blocking
    if let Ok(ip) = hostname.parse::<Ipv4Addr>() {
        return stream::once(future::ready(Ok(SocketAddr::new(ip.into(), port)))).boxed();
    }

    if let Ok(ip) = hostname.parse::<Ipv6Addr>() {
        return stream::once(future::ready(Ok(SocketAddr::new(ip.into(), port)))).boxed();
    }

    // slow path, use the Unblock struct with ToSocketAddrs
    let socket_addr = (hostname.to_string(), port);
    Unblock::new(move || std::net::ToSocketAddrs::to_socket_addrs(&socket_addr))
        .map(|res| res.map_err(Error::io))
        .boxed()
}

struct SocketDetails {
    addr: SockAddr,
    domain: Domain,
    protocol: Option<Protocol>,
}

enum SocketMode {
    Tcp,
    #[cfg(unix)]
    Unix,
}

impl SocketDetails {
    fn connect(self) -> Result<Socket> {
        let SocketDetails {
            addr,
            domain,
            protocol,
        } = self;

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
            Err(err) if err.raw_os_error() == Some(nix::libc::EINPROGRESS) => {}
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(err) => return Err(Error::io(err)),
        }
        Ok(socket)
    }
}
