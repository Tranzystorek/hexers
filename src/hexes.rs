use std::iter::FusedIterator;

use crate::aux::NibbleState;

fn to_hex(n: u8) -> Option<char> {
    std::char::from_digit(n as u32, 16)
}

/// Adaptor for an Iterator<Item = u8> that encodes every byte into hex chars.
pub struct Hexes<T> {
    bytes: T,
    front: Option<NibbleState>,
    back: Option<NibbleState>,
}

impl<T> Hexes<T> {
    fn try_next(&mut self) -> Option<char> {
        Self::get_next(&mut self.front)
    }

    fn try_next_last(&mut self) -> Option<char> {
        Self::get_next(&mut self.back)
    }

    fn try_next_back(&mut self) -> Option<char> {
        Self::get_next_back(&mut self.back)
    }

    fn try_next_back_last(&mut self) -> Option<char> {
        Self::get_next_back(&mut self.front)
    }

    fn get_next(opt: &mut Option<NibbleState>) -> Option<char> {
        opt.as_mut().and_then(|st| st.next()).and_then(to_hex)
    }

    fn get_next_back(opt: &mut Option<NibbleState>) -> Option<char> {
        opt.as_mut().and_then(|st| st.next_back()).and_then(to_hex)
    }
}

impl<T: Iterator<Item = u8>> Hexes<T> {
    pub fn from(iter: T) -> Self {
        Self {
            bytes: iter,
            front: None,
            back: None,
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Hexes<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next()
            .or_else(|| {
                self.front = self.bytes.next().map(NibbleState::from_byte);
                self.try_next()
            })
            .or_else(|| self.try_next_last())
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
                self.back = self.bytes.next_back().map(NibbleState::from_byte);
                self.try_next_back()
            })
            .or_else(|| self.try_next_back_last())
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
