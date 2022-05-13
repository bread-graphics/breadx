// MIT/Apache2 License

/// A type that can be used in the "void" position.
pub trait Void {
    /// The raw bytes of this item.
    fn bytes(&self) -> &[u8];
}

impl<T: bytemuck::NoUninit> Void for T {
    fn bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

impl<T: bytemuck::NoUninit> Void for [T] {
    fn bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }
}

impl Void for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}
