use crate::agents::minimax::MinimaxAgent;

//     ------------------------------------
//     |                |                 |
//     |    --------------------------    |
//     |    |           |            |    |
//     |    |    ----------------    |    |
//     |    |    |              |    |    |
//     |    |    |              |    |    |
//     |----|----|              |----|----|
//     |    |    |              |    |    |
//     |    |    |              |    |    |
//     |    |    ----------------    |    |
//     |    |           |            |    |
//     |    --------------------------    |
//     |                |                 |
//     ------------------------------------
pub const NUMBER_OF_RINGS: u8 = 3; // concentric "circles" of the game board
pub const NUMBER_OF_ALIGNMENTS: u8 = 8; // leading lines into the center on which pieces reside - don't change
pub const TOTAL_NUMBER_FIELDS: u8 = NUMBER_OF_ALIGNMENTS * NUMBER_OF_RINGS;
pub const PIECES_PER_TEAM: u8 = 9;

pub type AGENT = MinimaxAgent;

pub const DEFAULT_MODE: &str = "ai";