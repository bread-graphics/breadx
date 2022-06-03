// MIT/Apache2 License

use super::{Connection, ReadHalf, WriteHalf};
use crate::Result;

/// A type of connection that can be split into a write half and a read half.
///
/// This allows for certain types of [`Display`]s to, for instance, read an
/// event while also writing to the same connection.
///
/// [`Display`]: crate::Display
pub trait SplitConnection: Connection {
    /// The read half that this connection splits into.
    type ReadHalf: ReadHalf;
    /// The write half that this connection splits into.
    type WriteHalf: WriteHalf;

    /// Split this connection into its read and write halves.
    fn split(self) -> Result<(Self::ReadHalf, Self::WriteHalf)>;
}

/// A connection that can be cloned, potentially with an error.
///
/// This simplifies implementation of [`SplitConnection`] by allowing the
/// [`SplitConnection`] to be implemented for types that can be cloned.
///
/// [`SplitConnection`]: crate::connection::SplitConnection
pub trait ClonableConnection: Connection + Sized {
    /// Try to clone this connection.
    fn try_clone(&self) -> Result<Self>;
}

impl<C: Connection + Clone> ClonableConnection for C {
    fn try_clone(&self) -> Result<Self> {
        Ok(self.clone())
    }
}

impl<C: ClonableConnection> SplitConnection for C {
    type ReadHalf = C;
    type WriteHalf = C;

    fn split(self) -> Result<(Self::ReadHalf, Self::WriteHalf)> {
        let clone = self.try_clone()?;
        Ok((clone, self))
    }
}
