// MIT/Apache2 License

//! This module defines the `NameConnection` type, which is the default connection used for `Display` objects.
//! See the `NameConnection` object for more information.

#![cfg(feature = "std")]

use super::Connection;
use crate::Fd;
use alloc::{borrow::Cow, format, string::String, vec::Vec};
use core::mem;
use memchr::memrchr;
use std::{env, net, path::Path};

#[cfg(feature = "async")]
use super::{AsyncConnection, GenericConnFuture};
#[cfg(feature = "async")]
use core::task::{Context, Poll};

#[cfg(test)]
use std::borrow::ToOwned;

#[cfg(unix)]
use std::os::unix::net as unet;

#[cfg(all(feature = "async", unix))]
use async_net::unix as async_unet;

/// This is a wrapper around the connection created by `DisplayConnection::create()`. It implements
/// `Connection` for a variety of connections that X11 usually transmits itself over. You rarely
/// have to worry about this type of connection.
pub enum NameConnection {
    #[doc(hidden)]
    Tcp(net::TcpStream),
    #[cfg(unix)]
    #[doc(hidden)]
    Socket(unet::UnixStream),
}

impl Connection for NameConnection {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> crate::Result {
        match self {
            Self::Tcp(t) => t.send_packet(bytes, fds),
            #[cfg(unix)]
            Self::Socket(s) => s.send_packet(bytes, fds),
        }
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], fds: &mut Vec<Fd>) -> crate::Result {
        match self {
            Self::Tcp(t) => t.read_packet(bytes, fds),
            #[cfg(unix)]
            Self::Socket(s) => s.read_packet(bytes, fds),
        }
    }
}

#[cfg(feature = "async")]
pub enum AsyncNameConnection {
    #[doc(hidden)]
    Tcp(async_net::TcpStream),
    #[cfg(unix)]
    #[doc(hidden)]
    Socket(async_unet::UnixStream),
}

#[cfg(feature = "async")]
impl AsyncConnection for AsyncNameConnection {
    #[inline]
    fn poll_send_packet(
        &mut self,
        bytes: &mut &[u8],
        fds: &mut Vec<Fd>,
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result> {
        match self {
            Self::Tcp(t) => t.poll_send_packet(bytes, fds, cx),
            #[cfg(unix)]
            Self::Socket(s) => s.poll_send_packet(bytes, fds, cx),
        }
    }

    #[inline]
    fn poll_read_packet(
        &mut self,
        bytes: &mut &mut [u8],
        fds: &mut Vec<Fd>,
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result> {
        match self {
            Self::Tcp(t) => t.poll_read_packet(bytes, fds, cx),
            #[cfg(unix)]
            Self::Socket(s) => s.poll_read_packet(bytes, fds, cx),
        }
    }
}

/// Port for X11 server.
const X_TCP_PORT: u16 = 6000;

#[cfg(unix)]
const PART1: &str = "/tmp/.X11-unix/X";

/// The protocol used for the connection.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Protocol {
    Unix,
    Tcp,
    Inet,
    Inet6,
}

impl Protocol {
    #[allow(clippy::unnecessary_wraps)]
    #[inline]
    fn from_str(s: String) -> Option<Self> {
        let s = s.to_lowercase();
        Some(match s.as_str() {
            "unix" => Self::Unix,
            "tcp" => Self::Tcp,
            "inet" => Self::Inet,
            "inet6" => Self::Inet6,
            _ => {
                #[cfg(debug_assertions)]
                panic!("Unrecognized protocol: {}", s);
                #[cfg(not(debug_assertions))]
                return None;
            }
        })
    }
}

/// A connection to the X11 server.
#[derive(Debug, Clone, Eq, PartialEq)]
struct XConnection<'a> {
    host: Option<Cow<'a, str>>,
    protocol: Option<Protocol>,
    display: u16,
    screen: usize,
}

impl<'a> XConnection<'a> {
    // load the xconnection from a socket
    #[inline]
    fn parse_from_socket(name: Cow<'a, str>) -> Result<Self, Cow<'a, str>> {
        // if the name is a file path, use it as the host
        let (host, screen) = if Path::new(&*name).exists() {
            (name, 0)
        } else {
            // cut off the extension at the end and try again
            let rposn = match memrchr(b'.', name.as_bytes()) {
                Some(rposn) => rposn,
                None => return Err(name),
            };
            let screen = &name[rposn + 1..];
            let screen: usize = match screen.parse() {
                Ok(s) => s,
                Err(_) => return Err(name),
            };
            if Path::new(&name[..rposn]).exists() {
                (
                    match name {
                        Cow::Borrowed(s) => Cow::Borrowed(&s[..rposn]),
                        Cow::Owned(mut sr) => {
                            sr.truncate(rposn);
                            Cow::Owned(sr)
                        }
                    },
                    screen,
                )
            } else {
                return Err(name);
            }
        };

        Ok(XConnection {
            host: Some(host),
            protocol: Some(Protocol::Unix),
            display: 0,
            screen,
        })
    }

    /// Parse an `XConnection` from a name.
    pub fn parse(name: Option<Cow<'a, str>>) -> crate::Result<XConnection> {
        let name = match name {
            Some(name) => name,
            None => Cow::Owned(
                env::var("DISPLAY").map_err(|_| crate::BreadError::UnableToParseConnection)?,
            ),
        };

        // check if it is a socket first
        let mut name = match Self::parse_from_socket(name) {
            Ok(sock) => return Ok(sock),
            Err(name) => name,
        };

        // if the name isn't a socket, it's in the format of:
        //    [protocol/][host]:display[.screen]
        //
        // we parse it by first checking if there is a slash in the string, and then using
        // anything before that as the protocol. then we use an iterator and a fold to determine
        // the display, host, and screen. finally, we parse the display and screen

        // see if there is a slash in the name
        let protocol = match memrchr(b'/', name.as_bytes()) {
            Some(posn) => {
                let mut protocol = name.to_mut().split_off(posn + 1);
                mem::swap(name.to_mut(), &mut protocol);
                protocol.pop();
                Protocol::from_str(protocol)
            }
            None => None,
        };

        // now find the host
        let host = match memrchr(b':', name.as_bytes()) {
            None => return Err(crate::BreadError::UnableToParseConnection),
            Some(0) => None,
            Some(brek) => Some(match name {
                Cow::Borrowed(s) => Cow::Borrowed(&s[brek..]),
                Cow::Owned(ref mut sr) => {
                    let mut part = sr.split_off(brek);
                    mem::swap(sr, &mut part);
                    Cow::Owned(part)
                }
            }),
        };

        // iterate over
        let mut _dummy: String = String::new();
        let mut display: String = String::with_capacity(2);
        let mut screen: String = String::new();
        let mut current_target: &mut String = &mut _dummy;

        for c in name.chars() {
            match c {
                ':' => {
                    current_target = &mut display;
                }
                '.' => {
                    current_target = &mut screen;
                }
                c => {
                    current_target.push(c);
                }
            }
        }

        let display: u16 = if display.is_empty() {
            return Err(crate::BreadError::UnableToParseConnection);
        } else {
            display
                .parse()
                .map_err(|_| crate::BreadError::UnableToParseConnection)?
        };

        let screen = if screen.is_empty() {
            0
        } else {
            screen
                .parse()
                .map_err(|_| crate::BreadError::UnableToParseConnection)?
        };

        Ok(XConnection {
            host,
            protocol,
            display,
            screen,
        })
    }

    /// Split self into host and port.
    #[inline]
    fn host_and_port(self) -> (Cow<'a, str>, u16) {
        let XConnection { host, display, .. } = self;
        let host = match host {
            None => Cow::Borrowed("127.0.0.1"),
            Some(host) => host,
        };

        // the port will be X_TCP_PORT + display
        let port = X_TCP_PORT + display;
        (host, port)
    }

    /// Open the connection via TCP.
    #[inline]
    fn open_tcp(self) -> crate::Result<NameConnection> {
        let (host, port) = self.host_and_port();
        let connection = net::TcpStream::connect((&*host, port))?;
        Ok(NameConnection::Tcp(connection))
    }

    /// Derive the desired filename.
    #[cfg(unix)]
    #[inline]
    fn socket_filename(self) -> crate::Result<Cow<'a, str>> {
        let XConnection { host, .. } = self;
        match host {
            Some(host) => Ok(host),
            None => Err(crate::BreadError::UnableToOpenConnection),
        }
    }

    /// Open a socket file on Unix.
    #[cfg(unix)]
    #[inline]
    fn open_unix(self) -> crate::Result<NameConnection> {
        let fname = self.socket_filename()?;
        Ok(NameConnection::Socket(unet::UnixStream::connect(&*fname)?))
    }

    /// Open the connection.
    #[allow(unused_mut)]
    pub fn open(mut self) -> crate::Result<NameConnection> {
        // if the protocol or hostname isn't "unix", just run the tcp code
        if self.protocol != Some(Protocol::Unix)
            || (self.host.is_none() || self.host.as_deref().unwrap() != "unix")
        {
            if let Ok(c) = self.clone().open_tcp() {
                return Ok(c);
            }
        }

        // the next part only applies with unix semantics
        #[cfg(unix)]
        {
            if let Ok(u) = self.clone().open_unix() {
                return Ok(u);
            }

            // add the display name to the host
            self.host = Some(Cow::Owned(format!("{}{}", PART1, self.display)));
            self.open_unix()
        }

        // something wrong happened
        #[cfg(not(unix))]
        Err(crate::BreadError::UnableToOpenConnection)
    }

    /// Open the connection via TCP, async redox.
    #[cfg(feature = "async")]
    #[inline]
    async fn open_tcp_async(self) -> crate::Result<AsyncNameConnection> {
        let (host, port) = self.host_and_port();
        let connection = async_net::TcpStream::connect((&*host, port)).await?;
        Ok(AsyncNameConnection::Tcp(connection))
    }

    /// Open a socket file on Unix, async redox.
    #[cfg(all(feature = "async", unix))]
    async fn open_unix_async(self) -> crate::Result<AsyncNameConnection> {
        let fname = self.socket_filename()?;
        Ok(AsyncNameConnection::Socket(
            async_unet::UnixStream::connect(&*fname).await?,
        ))
    }

    /// Open an asynchronous connection.
    #[allow(unused_mut)]
    #[cfg(feature = "async")]
    pub async fn open_async(mut self) -> crate::Result<AsyncNameConnection> {
        // if the protocol or hostname isn't "unix", just run the tcp code
        if self.protocol != Some(Protocol::Unix)
            || (self.host.is_none() || self.host.as_deref().unwrap() != "unix")
        {
            if let Ok(c) = self.clone().open_tcp_async().await {
                return Ok(c);
            }
        }

        // the next part only applies with unix semantics
        #[cfg(unix)]
        {
            if let Ok(u) = self.clone().open_unix_async().await {
                return Ok(u);
            }

            // add the display name to the host
            self.host = Some(Cow::Owned(format!("{}{}", PART1, self.display)));
            self.open_unix_async().await
        }

        // something wrong happened
        #[cfg(not(unix))]
        Err(crate::BreadError::UnableToOpenConnection)
    }
}

impl NameConnection {
    /// Open a new connection.
    #[inline]
    pub(crate) fn connect_internal(
        name: Option<Cow<'_, str>>,
    ) -> crate::Result<(NameConnection, usize)> {
        let connection = XConnection::parse(name)?;
        let screen = connection.screen;
        Ok((connection.open()?, screen))
    }
}

#[cfg(feature = "async")]
impl AsyncNameConnection {
    /// Open a new asynchronous connection.
    #[inline]
    #[cfg(feature = "async")]
    pub(crate) async fn connect_internal_async(
        name: Option<Cow<'_, str>>,
    ) -> crate::Result<(AsyncNameConnection, usize)> {
        let connection = XConnection::parse(name)?;
        let screen = connection.screen;
        Ok((connection.open_async().await?, screen))
    }
}

#[cfg(test)]
macro_rules! borrowed_test {
    ($name: expr, $res: expr) => {{
        let xconn = XConnection::parse(Some(Cow::Borrowed($name))).unwrap();
        assert_eq!(xconn, ($res), "input: {}", $name);

        let xconn = XConnection::parse(Some(Cow::Owned(($name).to_owned()))).unwrap();
        assert_eq!(xconn, ($res), "input: {}", $name);
    }};
}

#[test]
fn parse_basic_display() {
    // basic display
    borrowed_test!(
        ":3",
        XConnection {
            host: None,
            protocol: None,
            screen: 0,
            display: 3
        }
    );
}

#[test]
fn parse_display_and_screen() {
    // display with screen
    borrowed_test!(
        ":3.6",
        XConnection {
            host: None,
            protocol: None,
            screen: 6,
            display: 3
        }
    );
}

#[test]
fn parse_display_screen_and_protocol() {
    // display with screen and protocol
    let xconn = XConnection::parse(Some(Cow::Borrowed("inet/:5"))).unwrap();
    assert_eq!(
        xconn,
        XConnection {
            host: None,
            protocol: Some(Protocol::Inet),
            screen: 0,
            display: 5
        }
    );

    for (protocol, res) in &[
        ("unix", Protocol::Unix),
        ("inet", Protocol::Inet),
        ("inet6", Protocol::Inet6),
        ("tcp", Protocol::Tcp),
    ] {
        let xconn = XConnection::parse(Some(Cow::Owned(format!("{}/:9.2", protocol)))).unwrap();
        assert_eq!(
            xconn,
            XConnection {
                host: None,
                protocol: Some(*res),
                screen: 2,
                display: 9
            }
        );
    }
}

#[should_panic]
#[test]
fn parse_arbitrary() {
    XConnection::parse(Some(Cow::Borrowed("arbitrary"))).unwrap();
}
