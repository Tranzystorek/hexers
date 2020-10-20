use std::iter::FusedIterator;

use crate::aux::NibbleState;

/// Adaptor for an Iterator<Item = u8> that encodes every byte into its nibbles (high-to-low order).
pub struct Nibbles<T> {
    bytes: T,
    front: Option<NibbleState>,
    back: Option<NibbleState>,
}

impl<T> Nibbles<T> {
    fn try_next(&mut self) -> Option<u8> {
        Self::get_next(&mut self.front)
    }

    fn try_next_last(&mut self) -> Option<u8> {
        Self::get_next(&mut self.back)
    }

    fn try_next_back(&mut self) -> Option<u8> {
        Self::get_next_back(&mut self.back)
    }

    fn try_next_back_last(&mut self) -> Option<u8> {
        Self::get_next_back(&mut self.front)
    }

    fn get_next(opt: &mut Option<NibbleState>) -> Option<u8> {
        opt.as_mut().and_then(|st| st.next())
    }

    fn get_next_back(opt: &mut Option<NibbleState>) -> Option<u8> {
        opt.as_mut().and_then(|st| st.next_back())
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

impl<T: DoubleEndedIterator<Item = u8>> DoubleEndedIterator for Nibbles<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.try_next_back()
            .or_else(|| {
                self.back = self.bytes.next_back().map(NibbleState::from_byte);
                self.try_next_back()
            })
            .or_else(|| self.try_next_back_last())
    }
}

impl<T: FusedIterator<Item = u8>> FusedIterator for Nibbles<T> {}
