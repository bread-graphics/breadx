// MIT/Apache2 License

//! In this series of tutorials, I will teach you how to use `breadx` and the X Window System Protocol to the
//! best of my ability. This tutorial assumes at least a beginner's knowledge of how to use Rust. I will not
//! attempt to teach you Rust in this tutorial; people who are much better at this kind of thing than I am
//! have already covered this subject extensively.
//! 
//! # Why you should use X
//! 
//! Here, I try to convince you that you, the reader, do not need to use this library. If you want to construct
//! a graphical application, [GTK+](https://www.gtk.org/) and [Qt](https://www.qt.io/) are very mature libraries
//! that support an astounding variety of platforms. Hell, we're living in 2020. The idea of a webpage has
//! replaced the GUI application, and they are usually much easier to set up. 
//! 
//! If you're still reading, I'm going to assume you fall into one of the following categories:
//! 
//! * **You want a smaller application size.** GTK+ and Qt are huge, monolithic libraries. Both of them use
//!   some kind of metaprogramming framework that can make code written in the framework almost alien to its
//!   mother language. Static linking with either library is almost impossible. Meanwhile, Xlib is a smalller
//!   library. If compatibility with non-Unix platforms is not a concern, using only `breadx` can lead to
//!   significantly smaller program sizes.
//! * **You're writing the GUI library.** Most GUI libraries use some kind of underlying interface to the
//!   system to draw their windows. For Unix-based systems, X is the dominant choice.
//! * **You're just interested in learning how X works.** There's nothing wrong with learning more about how
//!   your computer ticks.
//! 
//! In which case, welcome aboard. Let's go over what goes on behind the scenes.
//! 
//! # The X Window System Protocol
