enum HexState {
    Unread,
    LeftRead,
    RightRead
}

pub struct HexedByte<'a> {
    byte: &'a u8,
    state: HexState
}

impl HexState {
    pub fn advance(&mut self) {
        *self = match self {
            HexState::Unread => HexState::LeftRead,
            _ => HexState::RightRead
        }
    }
}

impl<'a> HexedByte<'a> {
    pub fn from_ref(byte: &'a u8) -> HexedByte<'a> {
        HexedByte::<'a> {
            byte,
            state: HexState::Unread
        }
    }

    fn get_left(&self) -> u8 {
        self.byte >> 4
    }

    fn get_right(&self) -> u8 {
        self.byte & 0xf
    }
}

impl<'a> Iterator for HexedByte<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.state {
            HexState::Unread => Some(self.get_left()),
            HexState::LeftRead => Some(self.get_right()),
            HexState::RightRead => None
        };

        self.state.advance();

        result
    }
}

impl<'a> DoubleEndedIterator for HexedByte<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result = match self.state {
            HexState::Unread => Some(self.get_right()),
            HexState::LeftRead => Some(self.get_left()),
            HexState::RightRead => None
        };

        self.state.advance();

        result
    }
}
