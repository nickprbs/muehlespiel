use std::mem::size_of;
use std::sync::{Arc, Mutex, RwLock};
use std::{thread, time};
use std::sync::mpsc::{channel, Receiver};
use std::time::SystemTime;
use fnv::{FnvBuildHasher, FnvHashMap};
use itertools::Itertools;
use crate::ai::agent::Agent;
use crate::ai::evaluation::evaluate_position;
use crate::datastructures::{BoardHistory, BoardHistoryMap, CanonicalBoardSet, Encodable, GameBoard, Phase, Team, Turn, WonLostMap};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::datastructures::Phase::{MOVE, PLACE};
use crate::datastructures::Team::{BLACK, WHITE};
use crate::iterators::ChildTurnIterator;

pub struct MinimaxAgent {}

const DEPTH_LIMIT: u16 = u16::MAX;
const ALPHA: f32 = 1.1;
const BETA: f32 = 2.42;

#[derive(Debug, Hash, Eq, PartialEq)]
struct TranspositionTableKey(u16, Turn);

type TranspositionTableValue = f32;

const TRANSPOSITION_ENTRY_SIZE: usize = size_of::<TranspositionTableKey>() + size_of::<TranspositionTableValue>() + 32;
const TRANSPOSITION_TABLE_CAPACITY: usize = 100_000_000;

// 1,000,000,000 is roughly 2GB
type TranspositionTable = FnvHashMap<TranspositionTableKey, TranspositionTableValue>;

impl Agent for MinimaxAgent {
    fn get_next_move(
        team: Team,
        board: GameBoard,
        history: Arc<Mutex<impl BoardHistory + 'static>>,
        lost_states_for_white: Arc<RwLock<WonLostMap>>,
        won_states_for_black: Arc<RwLock<WonLostMap>>,
        num_invocations: usize, // How many times have we called this function before?
    ) -> Turn {
        let start_time = SystemTime::now();

        let (kill_signal_tx, kill_signal_rx) = channel();

        let current_best_move: Option<Turn> = None;
        let current_best_move_mutex = Arc::new(Mutex::new(current_best_move));

        let runner_best_move = Arc::clone(&current_best_move_mutex);
        // Thread that executes minimax with iterative deepening
        thread::Builder::new()
            .name("minimax_runner".to_string())
            .stack_size(TRANSPOSITION_ENTRY_SIZE * TRANSPOSITION_TABLE_CAPACITY + 2 * 1024 * 1024 * 1024)
            .spawn(move || {
                let placings_left = 18_usize.saturating_sub(num_invocations) as u8;
                Self::mini_max_root(
                    kill_signal_rx,
                    placings_left,
                    team,
                    board,
                    ALPHA,
                    BETA,
                    Arc::clone(&lost_states_for_white),
                    Arc::clone(&won_states_for_black),
                    Arc::clone(&history),
                    Arc::clone(&runner_best_move)
                );
                eprintln!("> Minimax runner stopped");
            }).expect("Couldn't run minimax");

        // Wait until we used up most of the time
        thread::sleep(time::Duration::from_millis(990));

        let main_thread_best_move = Arc::clone(&current_best_move_mutex);
        let current_best_move = main_thread_best_move.lock()
            .expect("Expected there to be a best move, but there wasn't")
            .clone();
        return match current_best_move {
            None => {
                eprintln!("> THERE WAS NO MOVE! This should NEVER happen! Returning some move quickly...");
                let phase = if 18_usize.saturating_sub(num_invocations) == 0 {
                    MOVE
                } else { Phase::PLACE };

                let any_turn = ChildTurnIterator::new(phase, team, board)
                    .nth(0)
                    .expect("There is no turn to do anymore");

                println!("{}", any_turn.encode());

                any_turn
            }
            Some(current_best_move) => {
                println!("{}", current_best_move.encode());
                eprintln!("> Took {}ms", start_time.elapsed().unwrap().as_millis());

                // Tell the runner that its time to stop
                // Ignore failures, since the thread could stop earlier, if it reached max depth for example
                let _ = kill_signal_tx.send(());

                current_best_move
            }
        };
    }
}

impl MinimaxAgent {
    fn mini_max_root(
        kill_signal: Receiver<()>,
        num_placings_left: u8,
        team_to_maximize: Team,
        board: GameBoard,
        alpha: f32, // lower bound (this move is so bad that all it's children are probably too)
        beta: f32,  // upper bound (this move is op, take it immediately for this subtree!)
        lost_states_for_white: Arc<RwLock<WonLostMap>>,
        won_states_for_black: Arc<RwLock<WonLostMap>>,
        history: Arc<Mutex<impl BoardHistory>>,
        current_best_move_mutex: Arc<Mutex<Option<Turn>>>,
    ) {
        let phase = if num_placings_left == 0 {
            MOVE
        } else {
            PLACE
        };

        if phase == PLACE {
            Self::mini_max_iterative_deepening(
                kill_signal,
                num_placings_left,
                team_to_maximize,
                board,
                alpha,
                beta,
                lost_states_for_white,
                won_states_for_black,
                history,
                current_best_move_mutex,
            );
            return;
        }

        let turns = ChildTurnIterator::new(
            phase,
            team_to_maximize,
            board,
        );

        let killer_turns: Vec<(Turn, u16)> = turns
            .filter_map(|turn| {
                let board_after_turn = board.apply(turn.clone(), team_to_maximize);
                let team_after_turn = team_to_maximize.get_opponent();
                let representative_as_white = match team_after_turn {
                    WHITE => board_after_turn.get_representative(),
                    BLACK => board_after_turn.invert_teams().get_representative()
                };
                let representative_as_black = match team_after_turn {
                    BLACK => board_after_turn.get_representative(),
                    WHITE => board_after_turn.invert_teams().get_representative()
                };
                let lost_states_for_white = lost_states_for_white.read().unwrap();
                let lost_record = lost_states_for_white.get(&representative_as_white);
                let opponent_lost = lost_record.is_some();
                let is_killer = opponent_lost;
                let is_tie = history.lock().unwrap().will_be_tie(board_after_turn);

                if is_killer && !is_tie {
                    Some((turn, *lost_record.unwrap()))
                } else {
                    None
                }
            })
            .collect();

        if killer_turns.len() > 0 {
            let min_distance_turn = killer_turns.iter().min_by_key(|(_turn, distance)| distance);
            let any_killer_turn = min_distance_turn.unwrap().0.clone();
            eprintln!("> Found a killer turn: {:?}", any_killer_turn);
            let mut current_best_move = current_best_move_mutex.lock().unwrap();
            current_best_move.replace(any_killer_turn);
        } else {
            Self::mini_max_iterative_deepening(
                kill_signal,
                num_placings_left,
                team_to_maximize,
                board,
                alpha,
                beta,
                lost_states_for_white,
                won_states_for_black,
                history,
                current_best_move_mutex,
            );
        }
    }

    fn mini_max_iterative_deepening(
        kill_signal_rx: Receiver<()>,
        num_placings_left: u8,
        team: Team,
        board: GameBoard,
        alpha: f32, // lower bound (this move is so bad that all it's children are probably too)
        beta: f32,  // upper bound (this move is op, take it immediately for this subtree!)
        lost_states_for_white: Arc<RwLock<WonLostMap>>,
        won_states_for_black: Arc<RwLock<WonLostMap>>,
        history: Arc<Mutex<impl BoardHistory>>,
        current_best_move_mutex: Arc<Mutex<Option<Turn>>>,
    ) {
        let mut max_depth: u16 = 1;

        eprintln!("> Initing transposition table: {}bytes * {}", TRANSPOSITION_ENTRY_SIZE, TRANSPOSITION_TABLE_CAPACITY);
        let mut transposition_table: TranspositionTable =
            FnvHashMap::with_capacity_and_hasher(TRANSPOSITION_TABLE_CAPACITY, FnvBuildHasher::default());

        // While we didn't receive a kill signal
        while !kill_signal_rx.try_recv().is_ok() {
            Self::mini_max(
                num_placings_left,
                team,
                board,
                0,
                max_depth,
                alpha,
                beta,
                &mut transposition_table,
                Arc::clone(&lost_states_for_white),
                Arc::clone(&won_states_for_black),
                Arc::clone(&history),
                Arc::clone(&current_best_move_mutex),
            );
            //eprintln!("Completed search with max depth {}", max_depth);
            if max_depth < DEPTH_LIMIT {
                max_depth += 1;
            } else {
                eprintln!("> Reached maximum depth");
                break;
            }
        }

    }

    fn mini_max(
        num_placings_left: u8,
        team_to_maximize: Team,
        board: GameBoard,
        depth: u16,
        max_depth: u16,
        alpha: f32, // lower bound (this move is so bad that all it's children are probably too)
        beta: f32,  // upper bound (this move is op, take it immediately for this subtree!)
        transposition_table: &mut TranspositionTable,
        lost_states_for_white: Arc<RwLock<WonLostMap>>,
        won_states_for_black: Arc<RwLock<WonLostMap>>,
        history: Arc<Mutex<impl BoardHistory>>,
        current_best_move_mutex: Arc<Mutex<Option<Turn>>>,
    ) -> f32 {
        let phase = if num_placings_left == 0 {
            MOVE
        } else {
            PLACE
        };
        let opponent = team_to_maximize.get_opponent();

        let representative_as_white = match team_to_maximize {
            BLACK => board.invert_teams().get_representative(),
            WHITE => board.get_representative()
        };

        return if depth == 0 && history.lock().unwrap().will_be_tie(board) {
            if lost_states_for_white.read().unwrap().contains_key(&representative_as_white) {
                1.0
            } else {
                0.0
            }
        } else if depth == max_depth || board.is_game_done() {
            evaluate_position(team_to_maximize, phase, board, depth)
        } else {
            let turns = ChildTurnIterator::new(
                phase,
                team_to_maximize,
                board,
            ).dedup()
                .sorted_unstable_by(|turn_a, turn_b| {
                    // TODO: Test if sorting works
                    // Reverse, since we want to try those with higher evaluations first, so that pruning is more effective
                    transposition_table.get(&TranspositionTableKey(depth, turn_b.clone()))
                        .unwrap_or(&1.7_f32)
                        .partial_cmp(
                            transposition_table.get(&TranspositionTableKey(depth, turn_a.clone()))
                                .unwrap_or(&1.7_f32)
                        )
                        .unwrap()
                });

            let mut m = alpha;

            for turn in turns {
                let new_board = board.apply(turn.clone(), team_to_maximize);

                let result = 3.0 - Self::mini_max(
                    num_placings_left.saturating_sub(1),
                    opponent,
                    new_board,
                    depth + 1,
                    max_depth,
                    -beta - 0.2,
                    m - 0.2,
                    transposition_table,
                    Arc::clone(&lost_states_for_white),
                    Arc::clone(&won_states_for_black),
                    Arc::clone(&history),
                    Arc::clone(&current_best_move_mutex),
                );

                // Add the result to our transposition table
                {
                    let eighty_percent_capacity: f32 = 0.8f32 * (TRANSPOSITION_TABLE_CAPACITY as f32);
                    // Check if there's still room in the table. Don't fill it completely, since it will become *very* inefficient.
                    if (transposition_table.len() as f32) < eighty_percent_capacity {
                        transposition_table.insert(TranspositionTableKey(depth, turn.clone()), result);
                    }
                    // TODO: Make some strategy to replace values in transposition table if full
                }

                if result > m {
                    // Keep track of the best move (only on depth 1)
                    if depth == 0 {
                        let mut current_best_move = current_best_move_mutex.lock().unwrap();
                        if !current_best_move.eq(&Some(turn.clone())) {
                            let last_best = current_best_move.clone();
                            match last_best {
                                Some(last_best) => eprintln!("> Replacing current best move ({}) with new: {} (result: {result})", last_best.encode(), turn.encode()),
                                None => eprintln!("> Setting initial best move to {} (result: {result})", turn.encode()),
                            };
                            current_best_move.replace(turn);
                        }
                    }

                    m = result;

                    if result >= beta {
                        // We found a really good turn! Let's return it immediately.
                        break;
                    }
                }
            }

            m
        };
    }
}

#[test]
fn test_time_bounds() {
    let start = SystemTime::now();
    MinimaxAgent::get_next_move(
        WHITE,
        GameBoard::decode(String::from("WWWBBBEEEEEEEEEEEEEEEEEE")),
        Arc::new(Mutex::new(BoardHistoryMap::default())),
        Arc::new(RwLock::new(WonLostMap::default())),
        Arc::new(RwLock::new(WonLostMap::default())),
        20,
    );
    let duration = SystemTime::now().duration_since(start).expect("Time went backwards");

    println!("Took {}ms", duration.as_millis());

    assert!(duration.as_millis() < 1000);
}