mod hexed;
mod nibbles;

#[cfg(test)]
mod tests;

pub use hexed::Hexed;
pub use nibbles::Nibbles;
/// [`IntoIterator`] enabled version of [`Hexers::hexed`].
///
/// # Example
///
/// ```
/// use hexers::hexed;
///
/// let bytes = vec![0xbe, 0xef];
///
/// for hex in hexed(bytes) {
///     let _: char = hex;
/// }
/// ```
pub fn hexed<I>(iterable: I) -> Hexed<I::IntoIter>
where
    I: IntoIterator<Item = u8>,
{
    Hexed::from(iterable.into_iter())
}

/// [`IntoIterator`] enabled version of [`Hexers::nibbles`].
///
/// # Example
///
/// ```
/// use hexers::nibbles;
///
/// let bytes = vec![0xbe, 0xef];
///
/// for nib in nibbles(bytes) {
///     let _: u8 = nib;
/// }
/// ```
pub fn nibbles<I>(iterable: I) -> Nibbles<I::IntoIter>
where
    I: IntoIterator<Item = u8>,
{
    Nibbles::from(iterable.into_iter())
}

/// An [`Iterator`] blanket that provides the adaptor to hex sequences of bytes.
pub trait Hexers: Iterator {
    /// Creates an iterator over nibbles in the original sequence (high-to-low order).
    ///
    /// # Example
    ///
    /// ```
    /// use hexers::Hexers;
    ///
    /// let bytes = [0xbe, 0xef];
    /// let mut it = bytes.iter().copied().nibbles();
    ///
    /// assert_eq!(it.next(), Some(0xb));
    /// assert_eq!(it.next(), Some(0xe));
    /// assert_eq!(it.next(), Some(0xe));
    /// assert_eq!(it.next(), Some(0xf));
    /// ```
    fn nibbles(self) -> Nibbles<Self>
    where
        Self: Sized + Iterator<Item = u8>,
    {
        Nibbles::from(self)
    }

    /// Creates an iterator over hex encoded bytes in the original sequence.
    ///
    /// # Example
    ///
    /// ```
    /// use hexers::Hexers;
    ///
    /// let bytes = [0xbe, 0xef];
    /// let mut it = bytes.iter().copied().hexed();
    ///
    /// assert_eq!(it.next(), Some('b'));
    /// assert_eq!(it.next(), Some('e'));
    /// assert_eq!(it.next(), Some('e'));
    /// assert_eq!(it.next(), Some('f'));
    /// ```
    fn hexed(self) -> Hexed<Self>
    where
        Self: Sized + Iterator<Item = u8>,
    {
        Hexed::from(self)
    }
}

impl<T> Hexers for T where T: Iterator<Item = u8> {}
