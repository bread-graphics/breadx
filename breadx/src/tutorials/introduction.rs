//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

#![allow(clippy::doc_markdown)]

//! # An Introduction to X11
//!
//! Hello! My name is John, and I'm the author of `breadx`. While writing
//! `breadx`, I quickly realized that X11 is a confusing beast, so I had
//! to write something that explained it.
//!
//! Actually, that's not my real reason for writing this, or my real name
//! (or is it?) The real reason I'm writing this is because I haven't
//! been able to find a *good* X11 tutorial anywhere on the internet.
//! The best one I've been able to find is Cristopher Tronche's
//! translation of the [Xlib manual], and even then it's kind of hard to
//! follow. Tutorials for extensions are even more rare, to the point
//! that everything I've learned about extensions, I've learned orally.
//! That doesn't seem right for something as critical to the ecosystem
//! as X11.
//!
//! I'm going to try to put everything I know about X11 in this inline
//! documentation. I'll also try to make it parsable to the average
//! software hobbyist. This won't be a complete manual, nor will it be
//! entirely comprehensive, but it should be enough for anyone who wants
//! to learn X11.
//!
//! Before you start reading, grab a cup of water. If you've found yourself
//! this deep in the weeds, you're probably thirsty.
//!
//! [Xlib manual]: https://tronche.com/gui/x/xlib/
//!
//! ## Why X11?
//!
//! I'm going to put this disclaimer at the top here, for anyone who's
//! reading this casually: **you probably don't need to know X11**. It's
//! 2021. Higher level GUI frameworks have existed since GUIs themselves.
//! Not to mention, it's far more difficult to do anything through X11
//! than with these frameworks.
//!
//! So if you're just looking for something to construct your app with,
//! you won't find it here. Well, you *can* find it here. It's just that
//! it'll take a lot more effort to find, and that effort won't be worth it,
//! since it won't work on Windows.
//!
//! So, why *do* you need X11? Well, here are a handful of scattered reasons.
//!
//! - You want something close and low to the hardware. Although GUI
//!   frameworks have made excellent strides in being as low to the
//!   hardware as possible, there are some things directly tied to X11.
//! - You're *writing* a GUI framework, and need X11 to implement it.
//! - Your program needs a feature that your GUI framework doesn't offer,
//!   but an X11 extension does.
//! - You just want to learn something cool (although it escapes me
//!   why anyone would consider X11 to be trendy).
//!
//! I'm going to assume you fit into one of these categories. In which
//! case, welcome! I hope you find this tutorial useful for your usecase.
//!
//! ## A Brief History
//!
//! X began its life at MIT, as a sub-project of Project Athena. They
//! needed a windowing system, so like true programmers, they stole it.
//! X's name comes from the letter before W, which was the system they
//! modified that would eventually become X.
//!
//! The Digital Equipment Corporation then used X version 6 (abbreviated
//! to X6) in its Ultrix workstation. Although X6 originally required a
//! fee, by X9 the protocol was released under an open-source license that
//! would eventually become the [MIT License]. This model allowed corporations
//! like DEC and HP to integrate X into their products, causing X10 to become
//! widely spread. Eventually, X11 was released, building on previous versions
//! while also being significantly more hardware neutral.
//!
//! X11 required both a client and a server. Originally, OS developers
//! released their own versions of the X server, with [XFree86] being the
//! dominant option on Linux and the BSDs. However, due to certain controveries
//! over XFree86, the developers eventually migrated to the [X.Org] project.
//! To this day, you can find X.Org running on nearly every Linux desktop.
//!
//! But let's focus on the client. The original client library for X11 was
//! [Xlib]. Now, Xlib was very much a product of its time. By that I mean that,
//! by modern standards, Xlib is a dumpster fire. Namely:
//!
//! - The lack of support for sending requests in parallel.
//! - Providing little direct access to the X11 protocol.
//! - Containing a litany of code that no one really needed.
//! - Being bloated in general. In fact, the size of binaries statically
//!   linked to Xlib was an early incentive to use dynamic linking.
//! - Having an interface that is so arcane, programming in Xlib has
//!   become synonymous with dealing with dysfunctional APIs.
//! - Being known to contain errors, but being such a pain to debug that
//!   no one really tried to fix those errors.
//!
//! [XCB] was released in 2001 to address some of the above problems. It's
//! become the standard for programming X11 in the modern day. However,
//! many applications still use Xlib for legacy compatibility.
//!
//! `breadx` aims to take a similar approach to XCB: provide as direct of
//! access as possible to the underlying protocol as possible.
//!
//! [MIT License]: https://en.wikipedia.org/wiki/MIT_License
//! [Xlib]: https://en.wikipedia.org/wiki/Xlib
//! [XCB]: https://en.wikipedia.org/wiki/XCB
//! [XFree86]: https://en.wikipedia.org/wiki/XFree86
//! [X.Org]: https://en.wikipedia.org/wiki/X.Org
//!
//! ## What is X11?
//!
//! X11 is a communication protocol, similar to TCP or HTTP. It can take
//! place over "any reliable byte stream". However, in most cases, it's
//! used over TCP or a Unix socket. Communication takes place between a
//! client and a server. The client is your application, while the
//! server is a designated application with control over the hardware
//! involving your screen (most often X.org or Wayland).
//!
//! X11 opens with an initial handshake. The client sends a "setup request"
//! to the server, containing the client's endianness and authorization
//! information. In response, the server will either indicate a failure
//! state (one of the parameters is wrong) or send back a "setup". This
//! object contains initial information about the server, such as the
//! maximum request length, the sizes of attached screens, and the
//! preferred image bit ordering.
//!
//! From here, the client can send requests to the server. Requests
//! are things that the client wants to happen, such as "create a window",
//! "draw these trapezoids" or "copy data between two windows".
//!
//! The server, on the other hand, can send one of three types of
//! response:
//!
//! - **Replies** are responses to requests. They are sent back to the
//!   client in response to a request. Not all requests have replies.
//! - **Events** are responses to user events. For instance, clicking on
//!   a window will generate an event.
//! - **Errors** are responses to requests that failed.
//!
//! One of X11's major advantages is its fully asynchronous nature. You don't
//! have to wait for a response to send another request. You can, for instance,
//! send five requests, and then wait for each of those request's responses
//! in order to pipeline requests.
//!
//! At a broad level, that's it. There are some additional details, which we'll
//! go into in later chapters. But really, that's it.
//!
//! It's time to create an application, using what we know.
//!
//! [Next Tutorial](crate::tutorials::basics)
