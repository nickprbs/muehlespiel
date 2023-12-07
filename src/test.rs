#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use fnv::FnvBuildHasher;
    use crate::agents::Agent;
    use crate::agents::minimax::MinimaxAgent;
    use crate::Location;
    use crate::modes::enumerate_actions_from_file;
    use crate::types::{GameBoardHistoryCounter, GameBoardHistoryMap, GameContext};
    use crate::types::game_board::QueryableGameBoard;

    #[test]
    fn test_enumeration_id_to_location() {
        assert_eq!(
            Location::from_enumeration_id(0),
            Location { ring: 0, alignment: 0 }
        );
        assert_eq!(
            Location::from_enumeration_id(3),
            Location { ring: 0, alignment: 3 }
        );
        assert_eq!(
            Location::from_enumeration_id(12),
            Location { ring: 1, alignment: 4 }
        );
    }

    #[test]
    fn test_location_to_enumeration_id() {
        assert_eq!(
            Location { ring: 0, alignment: 0 }.to_enumeration_id(),
            0
        );
        assert_eq!(
            Location { ring: 0, alignment: 4 }.to_enumeration_id(),
            4
        );
        assert_eq!(
            Location { ring: 1, alignment: 3 }.to_enumeration_id(),
            11
        );
    }

    #[test]
    fn test_enumeration_e2e() {
        fs::write("./input_felder.txt", "WBEEEEWBEEEWEEWEWEBBBBBB\n\
                                                 BEEEEWWBBEWEWWEEBEBWEBEW\n\
                                                 EWWBBEEEEEWBBEEEEEEEBEEB\n\
                                                 BBWBEEEBBWBEWEBWBBEWEWEE\n\
                                                 EEWEWEEEEWWEWBBBEEWEEEEE\n\
                                                 BEEEBEWEBBEWEEBEEBBEBWEE\n\
                                                 BWEWBEWEBEEBBEBEBWEBEWBE\n\
                                                 WWEEWBBBBBEEWWEBWEEEEEEE").unwrap();
        enumerate_actions_from_file();
        assert_eq!(
            fs::read_to_string("./output.txt").unwrap(),
            "8 0 0\n\
             11 1 3\n\
             45 1 3\n\
             8 1 4\n\
             10 1 3\n\
             39 0 0\n\
             6 0 0\n\
             7 0 0"
        );
    }

    #[test]
    fn test_enumeration_e2e_large() {
        let input = fs::read_to_string("./input_felder_large.txt").unwrap();
        fs::write("./input_felder.txt", input).unwrap();

        enumerate_actions_from_file();

        assert_eq!(
            fs::read_to_string("./output.txt").unwrap(),
            fs::read_to_string("./output_large.txt").unwrap()
        )
    }
}