use std::iter::FusedIterator;

use crate::aux::HexedByte;

/// Adapter for an Iterator<Item = u8> that encodes every byte into a hex.
pub struct Hexes<T> {
    bytes: T,
    hex: Option<HexedByte>
}

/// Transforms an iterator into a Hexes sequence.
///
/// # Example
///
/// ```
/// use hexers::hexes;
///
/// let bytes = [0xbe_u8, 0xef_u8];
/// let mut it = hexes(bytes.iter().copied());
///
/// assert_eq!(it.next(), Some('b'));
/// assert_eq!(it.next(), Some('e'));
/// assert_eq!(it.next(), Some('e'));
/// assert_eq!(it.next(), Some('f'));
/// ```
pub fn hexes<T: Iterator<Item = u8>>(iter: T) -> Hexes<T> {
    Hexes::from_iter(iter)
}

impl<T> Hexes<T> {
    fn try_next(&mut self) -> Option<char> {
        self.hex.as_mut()
            .and_then(|hexed| hexed.next()
                .and_then(|hex| std::char::from_digit(hex as u32, 16)))
    }

    fn try_next_back(&mut self) -> Option<char> {
        self.hex.as_mut()
            .and_then(|hexed| hexed.next_back()
                .and_then(|hex| std::char::from_digit(hex as u32, 16)))
    }
}

impl<T: Iterator<Item = u8>> Hexes<T> {
    pub fn from_iter(iter: T) -> Self {
        Hexes {
            bytes: iter,
            hex: None
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Hexes<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next()
            .or_else(|| {
                self.hex = self.bytes.next()
                    .map(HexedByte::from_byte);

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
        self.try_next_back()
            .or_else(|| {
                self.hex = self.bytes.next_back()
                    .map(HexedByte::from_byte);

                self.try_next_back()
            })
    }
}

impl<T: FusedIterator<Item = u8>> FusedIterator for Hexes<T> {}
