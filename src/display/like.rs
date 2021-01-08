// MIT/Apache2 License

use super::{Display, Connection};

/// Represents an object that contains a Display and can successfully be aliased to a display.
/// 
/// In certain cases, it becomes necessary for an extension to attach its own data to the display connection.
/// The most notable cases of this are XRender and GLX. Doing this with composition in Rust would entail 
/// enclosing the Display inside of the struct. For instance:
/// 
/// ```
/// use breadx::Display;
/// 
/// struct MyExtensionDisplay<Conn> {
///     dpy: Display<Conn>,
///     other_data: OtherData,
/// } 
/// # struct OtherData;
/// ```
/// 
/// Of course, this would mean that you could only use one of these extensions at once. It would be nice if you
/// could put the extension display for one extension inside of the extension display of another. `DisplayLike`
/// is the solution to this problem. Extension displays should take objects of type `DisplayLike` and also
/// implement `DisplayLike` if possible.
/// 
/// `DisplayLike` is implemented for `Display`, as well as any of the extension displays provided by `breadx`.
/// 
/// # Example
/// 
/// ```no_run
/// use breadx::{Display, DisplayConnection, DisplayLike};
/// use std::sync::{Arc, Mutex};
/// 
/// struct Extension1Display<Dpy> {
///     dpy: Dpy,
///     data: u32,
/// }
/// 
/// struct Extension2Display<Dpy> {
///     // let's say Extension2 needs to work across the FFI boundary and thus needs
///     // to have interior mutability
///     dpy: Arc<Mutex<Dpy>>, 
/// }
/// 
/// impl<Dpy: DisplayLike> DisplayLike for Extension1Display<Dpy> {
///     type Conn = Dpy::Conn;
///     
///     #[inline]
///     fn display(&self) -> &Display<Dpy::Conn> {
///         self.dpy.display()
///     }
/// 
///     #[inline]
///     fn display_mut(&mut self) -> &mut Display<Dpy::Conn> {
///         self.dpy.display_mut()
///     }
/// }
/// 
/// // since it's impossible to get an &mut Dpy from an Arc<Mutex<Dpy>>, we don't implement
/// // DisplayLike for Extension2Display<Dpy>
/// // essentially, it's a "final consumer" of the Display
/// 
/// let conn = DisplayConnection::create(None, None).unwrap();
/// let conn = Extension1Display {
///     dpy: conn,
///     data: 14,
/// };
/// let conn = Extension2Display {
///     dpy: Arc::new(Mutex::new(conn)),
/// };
/// ```
pub trait DisplayLike {
    type Conn: Connection;

    fn display(&self) -> &Display<Self::Conn>;
    fn display_mut(&mut self) -> &mut Display<Self::Conn>;
}

impl<C: Connection> DisplayLike for Display<C> {
    type Conn = C;

    #[inline]
    fn display(&self) -> &Self {
        self
    }

    #[inline]
    fn display_mut(&mut self) -> &mut Self {
        self
    }
}
