use crate::datastructures::Encodable;
use crate::datastructures::Team::{BLACK, WHITE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Team {
    BLACK,
    WHITE
}

impl Team {
    pub(crate) fn as_binary(&self) -> u8 {
        match self {
            BLACK => 0b01,
            WHITE => 0b10,
        }
    }

    pub(crate) fn get_opponent(&self) -> Self {
        match self {
            BLACK => WHITE,
            WHITE => BLACK
        }
    }
}

impl Encodable for Team {
    fn encode(&self) -> String {
        todo!()
    }

    fn decode(string: String) -> Self {
        match string.as_str() {
            "B" => BLACK,
            "W" => WHITE,
            _ => { panic!("Unknown team") }
        }
    }
}