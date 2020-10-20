pub enum NibbleState {
    Both(u8),
    Lo(u8),
    Hi(u8),
    Empty,
}

impl NibbleState {
    pub fn from_byte(byte: u8) -> Self {
        NibbleState::Both(byte)
    }
}

impl Iterator for NibbleState {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            NibbleState::Both(b) => {
                *self = NibbleState::Lo(b);
                Some(b >> 4)
            }
            NibbleState::Lo(b) => {
                *self = NibbleState::Empty;
                Some(b & 0xf)
            }
            NibbleState::Hi(b) => {
                *self = NibbleState::Empty;
                Some(b >> 4)
            }
            _ => None,
        }
    }
}

impl DoubleEndedIterator for NibbleState {
    fn next_back(&mut self) -> Option<Self::Item> {
        match *self {
            NibbleState::Both(b) => {
                *self = NibbleState::Hi(b);
                Some(b & 0xf)
            }
            NibbleState::Hi(b) => {
                *self = NibbleState::Empty;
                Some(b >> 4)
            }
            NibbleState::Lo(b) => {
                *self = NibbleState::Empty;
                Some(b & 0xf)
            }
            _ => None,
        }
    }
}
