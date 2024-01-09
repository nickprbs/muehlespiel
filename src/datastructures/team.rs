#[derive(Clone, Copy, Debug)]
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

    pub(crate) fn get_opponent(&self) -> Self {
        match self {
            Team::BLACK => Team::WHITE,
            Team::WHITE => Team::BLACK
        }
    }
}