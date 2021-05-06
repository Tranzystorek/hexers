use std::iter::FusedIterator;

use crate::nibbles::Nibbles;

fn to_hex(n: u8) -> Option<char> {
    char::from_digit(n as u32, 16)
}

/// Adaptor for an `Iterator<Item = u8>` that encodes every byte into hex chars.
pub struct Hexed<T> {
    nibbles: Nibbles<T>,
}

impl<T: Iterator<Item = u8>> Hexed<T> {
    pub fn from(iter: T) -> Self {
        Self {
            nibbles: Nibbles::from(iter),
        }
    }
}

impl<T: Clone> Clone for Hexed<T> {
    fn clone(&self) -> Self {
        Self {
            nibbles: self.nibbles.clone(),
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Hexed<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.nibbles.next().and_then(to_hex)
    }
}

impl<T: DoubleEndedIterator<Item = u8>> DoubleEndedIterator for Hexed<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nibbles.next_back().and_then(to_hex)
    }
}

impl<T: FusedIterator<Item = u8>> FusedIterator for Hexed<T> {}
