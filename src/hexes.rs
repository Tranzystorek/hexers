use std::slice::Iter;
use std::iter::FusedIterator;

use crate::aux::HexedByte;

/// Iterator that transforms the underlying `&[u8]` sequence
/// into a lowercase hex encoded sequence.
pub struct Hexes<'a> {
    it: Iter<'a, u8>,
    hex: Option<HexedByte<'a>>
}

/// Returns an iterator over a lowercase hex encoded
/// representation of the given byte slice.
///
/// # Example
///
/// ```
/// use hexers::hexes;
///
/// let bytes = &[0xbe_u8, 0xef_u8];
/// let mut it = hexes(bytes);
///
/// assert_eq!(it.next(), Some('b'));
/// assert_eq!(it.next(), Some('e'));
/// assert_eq!(it.next(), Some('e'));
/// assert_eq!(it.next(), Some('f'));
/// ```
pub fn hexes(slice: &[u8]) -> Hexes {
    Hexes::from_slice(slice)
}

impl<'a> Hexes<'a> {
    pub fn from_slice(slice: &'a [u8]) -> Hexes<'a> {
        Hexes::<'a> {
            it: slice.iter(),
            hex: None
        }
    }

    pub fn from_iter(it: Iter<'a, u8>) -> Hexes<'a> {
        Hexes::<'a> {
            it,
            hex: None
        }
    }

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

impl<'a> Iterator for Hexes<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next()
            .or_else(|| {
                self.hex = self.it.next()
                    .map(HexedByte::from_ref);

                self.try_next()
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.it.len() * 2))
    }
}

impl<'a> DoubleEndedIterator for Hexes<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.try_next_back()
            .or_else(|| {
                self.hex = self.it.next_back()
                    .map(HexedByte::from_ref);

                self.try_next_back()
            })
    }
}

impl<'a> ExactSizeIterator for Hexes<'a> {}

impl<'a> FusedIterator for Hexes<'a> {}
