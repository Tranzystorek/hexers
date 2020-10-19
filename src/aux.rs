enum HexState {
    Unread,
    LeftRead,
    RightRead,
}

pub struct HexedByte {
    byte: u8,
    state: HexState,
}

impl HexState {
    pub fn advance(&mut self) {
        *self = match self {
            HexState::Unread => HexState::LeftRead,
            _ => HexState::RightRead,
        }
    }
}

impl HexedByte {
    pub fn from_byte(byte: u8) -> HexedByte {
        HexedByte {
            byte,
            state: HexState::Unread,
        }
    }

    fn get_left(&self) -> u8 {
        self.byte >> 4
    }

    fn get_right(&self) -> u8 {
        self.byte & 0xf
    }
}

impl Iterator for HexedByte {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.state {
            HexState::Unread => Some(self.get_left()),
            HexState::LeftRead => Some(self.get_right()),
            HexState::RightRead => None,
        };

        self.state.advance();

        result
    }
}

impl DoubleEndedIterator for HexedByte {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result = match self.state {
            HexState::Unread => Some(self.get_right()),
            HexState::LeftRead => Some(self.get_left()),
            HexState::RightRead => None,
        };

        self.state.advance();

        result
    }
}
