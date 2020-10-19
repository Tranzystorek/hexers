use std::iter::FusedIterator;

use crate::aux::HexedByte;

/// Adaptor for an Iterator<Item = u8> that encodes every byte into hex chars.
pub struct Hexes<T> {
    bytes: T,
    hex: Option<HexedByte>,
}

impl<T> Hexes<T> {
    fn try_next(&mut self) -> Option<char> {
        self.hex
            .as_mut()
            .and_then(|hexed| hexed.next())
            .and_then(|hex| std::char::from_digit(hex as u32, 16))
    }

    fn try_next_back(&mut self) -> Option<char> {
        self.hex
            .as_mut()
            .and_then(|hexed| hexed.next_back())
            .and_then(|hex| std::char::from_digit(hex as u32, 16))
    }
}

impl<T: Iterator<Item = u8>> Hexes<T> {
    pub fn from(iter: T) -> Self {
        Hexes {
            bytes: iter,
            hex: None,
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Hexes<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().or_else(|| {
            self.hex = self.bytes.next().map(HexedByte::from_byte);
            self.try_next()
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, greater) = self.bytes.size_hint();

        (lower * 2, greater.map(|val| val * 2))
    }
}

impl<T: DoubleEndedIterator<Item = u8>> DoubleEndedIterator for Hexes<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.try_next_back().or_else(|| {
            self.hex = self.bytes.next_back().map(HexedByte::from_byte);
            self.try_next_back()
        })
    }
}

impl<T: FusedIterator<Item = u8>> FusedIterator for Hexes<T> {}

/// An Iterator blanket that provides the adaptor to hex sequences of bytes.
pub trait HexIterator: Iterator {
    /// Creates an iterator over hex encoded bytes in the original sequence.
    ///
    /// # Example
    ///
    /// ```
    /// use hexers::HexIterator;
    ///
    /// let bytes = [0xbe_u8, 0xef_u8];
    /// let mut it = bytes.iter().copied().hexed();
    ///
    /// assert_eq!(it.next(), Some('b'));
    /// assert_eq!(it.next(), Some('e'));
    /// assert_eq!(it.next(), Some('e'));
    /// assert_eq!(it.next(), Some('f'));
    /// ```
    fn hexed(self) -> Hexes<Self>
    where
        Self: Sized + Iterator<Item = u8>,
    {
        Hexes::from(self)
    }
}

impl<T> HexIterator for T where T: Iterator<Item = u8> {}
