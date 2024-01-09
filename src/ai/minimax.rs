use std::mem::size_of;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use timer::{Timer};
use chrono::Duration;
use fnv::{FnvBuildHasher, FnvHashMap};
use itertools::Itertools;
use crate::ai::agent::Agent;
use crate::ai::evaluation::evaluate_position;
use crate::datastructures::{Encodable, GameBoard, Phase, Team, Turn};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::datastructures::Team::{WHITE, BLACK};
use crate::iterators::ChildTurnIterator;

pub struct MinimaxAgent {}

const ALPHA: f32 = 0.1;
const BETA: f32 = 1.7;

#[derive(Hash, Eq, PartialEq)]
struct  TranspositionTableKey (u16, Turn);
type TranspositionTableValue = f32;
const TRANSPOSITION_ENTRY_SIZE: usize = size_of::<TranspositionTableKey>() + size_of::<TranspositionTableValue>() + 32;
const TRANSPOSITION_TABLE_CAPACITY: usize = 100_000_000; // 1,000,000,000 is roughly 2GB
type TranspositionTable = FnvHashMap<TranspositionTableKey, TranspositionTableValue>;

impl Agent for MinimaxAgent {
    fn get_next_move(phase: Phase, team: Team, board: GameBoard, history: ()) -> Turn {
        let current_best_move: Option<Turn> = None;
        let current_best_move_mutex = Arc::new(Mutex::new(current_best_move));

        let (final_return_tx, final_return_rx) = channel();

        let timer = Timer::new();
        let watchdog_best_move = Arc::clone(&current_best_move_mutex);
        // The time watchdog returns the currently best result once time is running out
        timer.schedule_with_delay(Duration::milliseconds(950), move || {
            eprintln!("We used almost all remaining time (950ms). Return something.");

            let current_best_move = watchdog_best_move.lock()
                .unwrap().clone().expect("OH SHIT! I didn't find any best move. Now what?");

            final_return_tx.send(current_best_move).unwrap();
        });

        let runner_best_move = Arc::clone(&current_best_move_mutex);
        // Thread that executes minimax with iterative deepening
        thread::Builder::new()
            .name("minimax_runner".to_string())
            .stack_size(TRANSPOSITION_ENTRY_SIZE * TRANSPOSITION_TABLE_CAPACITY + 2 * 1024 * 1024 * 1024)
            .spawn(move || {
            let mut max_depth: u16 = 1;

            eprintln!("Initing transposition table: {}bytes * {}", TRANSPOSITION_ENTRY_SIZE, TRANSPOSITION_TABLE_CAPACITY);
            let mut transposition_table: TranspositionTable =
                FnvHashMap::with_capacity_and_hasher(TRANSPOSITION_TABLE_CAPACITY, FnvBuildHasher::default());
            eprintln!("Transposition table initialized");

            loop {
                Self::mini_max(phase, team, board, 0, max_depth, ALPHA, BETA, &mut transposition_table, history, &runner_best_move);
                eprintln!("Completed search with max depth {}", max_depth);
                if max_depth < u16::MAX {
                    max_depth += 1;
                } else {
                    break;
                }
            }
        }).expect("Couldn't run minimax");

        return final_return_rx.recv().unwrap();
    }
}

impl MinimaxAgent {
    fn mini_max(
        phase: Phase,
        team_to_maximize: Team,
        board: GameBoard,
        depth: u16,
        max_depth: u16,
        alpha: f32, // lower bound (this move is so bad that all it's children are probably too)
        beta: f32,  // upper bound (this move is op, take it immediately for this subtree!)
        transposition_table: &mut TranspositionTable,
        history: (),
        current_best_move_mutex: &Arc<Mutex<Option<Turn>>>
    ) -> f32 {
        let opponent = team_to_maximize.get_opponent();

        if /*TODO: history.is_tie(board)*/ false {
            return 0.2; // There is not much value in making a tie
        } else if depth == max_depth || board.is_game_done() {
            return evaluate_position(team_to_maximize, board);
        } else {
            let turns = ChildTurnIterator::new(
                phase,
                team_to_maximize,
                board
            ).sorted_unstable_by(|turn_a, turn_b| {
                // TODO: Test if sorting works
                // Reverse, since we want to try those with higher evaluations first, so that pruning is more effective
                transposition_table.get(&TranspositionTableKey(depth, turn_b.clone()))
                    .unwrap_or(&0.1_f32)
                    .partial_cmp(
                        transposition_table.get(&TranspositionTableKey(depth, turn_a.clone()))
                            .unwrap_or(&0.1_f32)
                    )
                    .unwrap()
            });

            let mut m = alpha;

            for turn in turns {
                let new_board = board.apply(turn.clone(), team_to_maximize);

                let new_phase = match phase {
                    Phase::MOVE => Phase::MOVE,
                    Phase::PLACE => {
                        if todo!() {
                            Phase::MOVE
                        } else {
                            Phase::PLACE
                        }
                    },
                };

                let result = -Self::mini_max(
                    new_phase,
                    opponent,
                    new_board,
                    depth + 1,
                    max_depth,
                    -beta,
                    m,
                    transposition_table,
                    history,
                    current_best_move_mutex
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
                    if depth == 1 {
                        let mut current_best_move = current_best_move_mutex.lock().unwrap();
                        *current_best_move = Some(turn);
                    }

                    if result >= beta {
                        // We found a really good turn! Let's return it immediately.
                        break;
                    } else {
                        m = result;
                    }
                }
            }

            return m;
        }
    }
}

#[test]
fn test_time_bounds() {
    let start = SystemTime::now();
    MinimaxAgent::get_next_move(
        Phase::MOVE,
        WHITE,
        GameBoard::decode(String::from("EEEEEEEEEEEEEEEEEEEEEEEE")),
        ()
    );
    let duration = SystemTime::now().duration_since(start).expect("Time went backwards");

    println!("Took {}ms", duration.as_millis());

    assert!(duration.as_millis() < 1000);
}