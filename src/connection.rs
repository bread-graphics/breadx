// MIT/Apache2 License

use alloc::{borrow::Cow, boxed::Box, string::String};
use core::mem;
use memchr::memrchr;
use std::{env, io::prelude::*, net, path::Path};

#[cfg(feature = "async")]
use tokio::{io::AsyncWrite, net as tokio_net};

/// Port for X11 server.
const X_TCP_PORT: u16 = 6000;

/// The protocol used for the connection.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Protocol {
    Unix,
    Tcp,
    Inet,
    Inet6,
}

impl Protocol {
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
#[derive(Debug, Eq, PartialEq)]
struct XConnection<'a> {
    host: Option<Cow<'a, str>>,
    protocol: Option<Protocol>,
    display: u16,
    screen: u64,
}

impl<'a> XConnection<'a> {
    // load the xconnection from a socket
    fn parse_from_socket(name: Cow<'a, str>) -> Result<Self, Cow<'a, str>> {
        // if the name is a file path, use it as the host
        let (host, screen) = if Path::new(&*name).exists() {
            (name, 0)
        } else {
            // cut off the extension at the end and try again
            let rposn = match name.rfind('.') {
                Some(rposn) => rposn,
                None => return Err(name),
            };
            let mut screen = &name[rposn + 1..];
            let screen: u64 = match screen.parse() {
                Ok(s) => s,
                Err(_) => return Err(name),
            };
            if Path::new(&name[..rposn]).exists() {
                (
                    match name {
                        Cow::Borrowed(s) => Cow::Borrowed(&s[..rposn]),
                        Cow::Owned(mut sr) => {
                            sr.truncate(memrchr(b'.', sr.as_bytes()).unwrap());
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

    /// Parse an XConnection from a name.
    pub fn parse(name: Option<Cow<'a, str>>) -> crate::Result<XConnection> {
        let name = match name {
            Some(name) => name,
            None => {
                Cow::Owned(env::var("DISPLAY").map_err(|_| crate::Error::UnableToParseConnection)?)
            }
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
            None => return Err(crate::Error::UnableToParseConnection),
            Some(0) => None,
            Some(brek) => Some(match name {
                Cow::Borrowed(s) => Cow::Borrowed(&s[s.rfind(':').unwrap()..]),
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
            return Err(crate::Error::UnableToParseConnection);
        } else {
            display
                .parse()
                .map_err(|_| crate::Error::UnableToParseConnection)?
        };

        let screen = if screen.is_empty() {
            0
        } else {
            screen
                .parse()
                .map_err(|_| crate::Error::UnableToParseConnection)?
        };

        Ok(XConnection {
            host,
            protocol,
            screen,
            display,
        })
    }

    /// Open the connection via TCP.
    #[inline]
    fn open_tcp(self) -> crate::Result<Box<dyn Write>> {
        let XConnection {
            host,
            protocol,
            screen,
            display,
        } = self;
        let host = match host {
            None => Cow::Borrowed("127.0.0.1"),
            Some(host) => host,
        };

        // the port will be X_TCP_PORT + display
        let port = X_TCP_PORT + display;

        let connection = net::TcpStream::connect((&*host, port))?;
        Ok(Box::new(connection))
    }

    /// Open a socket file on Unix.
    #[inline]
    fn open_unix(self) -> crate::Result<Box<dyn Write>> {
        let XConnection {
            host,
            protocol,
            screen,
            display,
        } = self;
        let host = match host {
            Some(host) => host,
            None => return Err(crate::Error::UnableToOpenConnection),
        };

        unimplemented!()
    }

    /// Open the connection.
    pub fn open(self) -> crate::Result<Box<dyn Write>> {
        // if the protocol or hostname isn't "unix", just run the tcp code
        if self.protocol != Some(Protocol::Unix)
            || (self.host.is_none() || self.host.as_deref().unwrap() != "unix")
        {
            return self.open_tcp();
        }

        // the next part only applies with unix semantics
        #[cfg(unix)]
        {
            return self.open_unix();
        }

        // something wrong happened
        #[cfg(not(unix))]
        Err(crate::Error::UnableToOpenConnection)
    }

    /// Open the connection via TCP, async redox.
    #[cfg(feature = "async")]
    #[inline]
    async fn open_tcp_async(self) -> crate::Result<Box<dyn AsyncWrite>> {
        let XConnection {
            host,
            protocol,
            screen,
            display,
        } = self;
        let host = match host {
            None => Cow::Borrowed("127.0.0.1"),
            Some(host) => host,
        };

        // the port will be X_TCP_PORT + display
        let port = X_TCP_PORT + display;

        let connection = tokio_net::TcpStream::connect((&*host, port)).await?;
        Ok(Box::new(connection))
    }

    /// Open a socket file on Unix, async redox.
    #[cfg(feature = "async")]
    async fn open_unix_async(self) -> crate::Result<Box<dyn AsyncWrite>> {
        let XConnection {
            host,
            protocol,
            screen,
            display,
        } = self;
        let host = match host {
            Some(host) => host,
            None => return Err(crate::Error::UnableToOpenConnection),
        };

        unimplemented!()
    }

    /// Open an asynchronous connection.
    #[cfg(feature = "async")]
    pub async fn open_async(self) -> crate::Result<Box<dyn AsyncWrite>> {
        // if the protocol or hostname isn't "unix", just run the tcp code
        if self.protocol != Some(Protocol::Unix)
            || (self.host.is_none() || self.host.as_deref().unwrap() != "unix")
        {
            return self.open_tcp_async().await;
        }

        // the next part only applies with unix semantics
        #[cfg(unix)]
        {
            return self.open_unix_async().await;
        }

        // something wrong happened
        #[cfg(not(unix))]
        Err(crate::Error::UnableToOpenConnection)
    }
}

/// Open a new connection.
#[inline]
pub fn connect_internal(name: Option<Cow<'_, str>>) -> crate::Result<Box<dyn Write>> {
    let connection = XConnection::parse(name)?;
    connection.open()
}

/// Open a new asynchronous connection.
#[inline]
#[cfg(feature = "async")]
pub async fn connect_internal_async(
    name: Option<Cow<'_, str>>,
) -> crate::Result<Box<dyn AsyncWrite>> {
    let connection = XConnection::parse(name)?;
    connection.open_async().await
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

#[test]
fn parse_display_host_screen_and_protocol() {
    // display with host, screen and protocol
    let xconn = XConnection::parse(Some(Cow::Borrowed("unix/127.6.6.7:27.65"))).unwrap();
    assert_eq!(
        xconn,
        XConnection {
            host: Some(Cow::Borrowed("127.6.6.7")),
            protocol: Some(Protocol::Unix),
            screen: 65,
            display: 27
        },
        "input: unix/127.6.6.7:27.65",
    );

    let xconn = XConnection::parse(Some(Cow::Owned("inet6/255.255.1.1:0".to_owned()))).unwrap();
    assert_eq!(
        xconn,
        XConnection {
            host: Some(Cow::Owned("255.255.1.1".to_owned())),
            protocol: Some(Protocol::Inet6),
            screen: 0,
            display: 0
        },
        "input: inet6/255.255.1.1:0",
    );
}

#[test]
fn parse_socket_filename() {
    let xconn = XConnection::parse(Some(Cow::Borrowed("/tmp/.X11-unix/X0"))).unwrap();
    assert_eq!(
        xconn,
        XConnection {
            host: Some(Cow::Borrowed("/tmp/.X11-unix/X0")),
            protocol: Some(Protocol::Unix),
            screen: 0,
            display: 0
        }
    );

    let xconn = XConnection::parse(Some(Cow::Owned("/tmp/.X11-unix/X0".to_owned()))).unwrap();
    assert_eq!(
        xconn,
        XConnection {
            host: Some(Cow::Owned("/tmp/.X11-unix/X0".to_owned())),
            protocol: Some(Protocol::Unix),
            screen: 0,
            display: 0
        }
    );
}

#[test]
fn parse_socket_filename_with_screen() {
    let xconn = XConnection::parse(Some(Cow::Borrowed("/tmp/.X11-unix/X0.5"))).unwrap();
    assert_eq!(
        xconn,
        XConnection {
            host: Some(Cow::Borrowed("/tmp/.X11-unix/X0")),
            protocol: Some(Protocol::Unix),
            screen: 5,
            display: 0
        }
    );

    let xconn = XConnection::parse(Some(Cow::Owned("/tmp/.X11-unix/X0.7".to_owned()))).unwrap();
    assert_eq!(
        xconn,
        XConnection {
            host: Some(Cow::Owned("/tmp/.X11-unix/X0".to_owned())),
            protocol: Some(Protocol::Unix),
            screen: 7,
            display: 0
        }
    );
}

#[should_panic]
#[test]
fn parse_arbitrary() {
    XConnection::parse(Some(Cow::Borrowed("arbitrary"))).unwrap();
}
