#[derive(Clone, Copy)]
pub enum Team {
    BLACK,
    WHITE
}

impl Team {
    fn as_char(&self) -> char {
        todo!()
    }

    pub(crate) fn as_binary(&self) -> u8 {
        match self {
            Team::BLACK => 0b01,
            Team::WHITE => 0b10,
        }
    }
}