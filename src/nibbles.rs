use std::array::IntoIter as ArrayIter;
use std::iter::FusedIterator;

/// Adaptor for an Iterator<Item = u8> that encodes every byte into its nibbles (high-to-low order).
pub struct Nibbles<T> {
    bytes: T,
    front: Option<ArrayIter<u8, 2>>,
    back: Option<ArrayIter<u8, 2>>,
}

impl<T> Nibbles<T> {
    fn try_next(&mut self) -> Option<u8> {
        self.front.as_mut().into_iter().flatten().next()
    }

    fn try_next_last(&mut self) -> Option<u8> {
        self.back
            .as_mut()
            .into_iter()
            .flat_map(Iterator::rev)
            .next()
    }

    fn try_next_back(&mut self) -> Option<u8> {
        self.back.as_mut().into_iter().flatten().next()
    }

    fn try_next_back_last(&mut self) -> Option<u8> {
        self.front
            .as_mut()
            .into_iter()
            .flat_map(Iterator::rev)
            .next()
    }
}

impl<T: Iterator<Item = u8>> Nibbles<T> {
    pub fn from(iter: T) -> Self {
        Self {
            bytes: iter,
            front: None,
            back: None,
        }
    }
}

impl<T: Iterator<Item = u8>> Iterator for Nibbles<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next()
            .or_else(|| {
                self.front = self.bytes.next().map(|b| ArrayIter::new([b >> 4, b & 0xf]));
                self.try_next()
            })
            .or_else(|| self.try_next_last())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, greater) = self.bytes.size_hint();

        (lower * 2, greater.map(|val| val * 2))
    }
}

impl<T: DoubleEndedIterator<Item = u8>> DoubleEndedIterator for Nibbles<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.try_next_back()
            .or_else(|| {
                self.back = self
                    .bytes
                    .next_back()
                    .map(|b| ArrayIter::new([b & 0xf, b >> 4]));
                self.try_next_back()
            })
            .or_else(|| self.try_next_back_last())
    }
}

impl<T: FusedIterator<Item = u8>> FusedIterator for Nibbles<T> {}
