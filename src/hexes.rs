use std::iter::FusedIterator;

use crate::nibbles::Nibbles;

fn to_hex(n: u8) -> Option<char> {
    std::char::from_digit(n as u32, 16)
}

/// Adaptor for an `Iterator<Item = u8>` that encodes every byte into hex chars.
pub struct Hexes<T> {
    nibbles: Nibbles<T>,
}

impl<T: Iterator<Item = u8>> Hexes<T> {
    pub fn from(iter: T) -> Self {
        Self {
            nibbles: Nibbles::from(iter),
        }
    }
}

impl<T: Clone> Clone for Hexes<T> {
    fn clone(&self) -> Self {
        Self {
            nibbles: self.nibbles.clone(),
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Hexes<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.nibbles.next().and_then(to_hex)
    }
}

impl<T: DoubleEndedIterator<Item = u8>> DoubleEndedIterator for Hexes<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nibbles.next_back().and_then(to_hex)
    }
}

impl<T: FusedIterator<Item = u8>> FusedIterator for Hexes<T> {}
