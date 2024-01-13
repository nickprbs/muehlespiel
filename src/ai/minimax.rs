use std::mem::size_of;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::sync::mpsc::channel;
use std::time::SystemTime;
use fnv::{FnvBuildHasher, FnvHashMap};
use itertools::Itertools;
use crate::ai::agent::Agent;
use crate::ai::evaluation::evaluate_position;
use crate::datastructures::{BoardHistory, BoardHistoryMap, Encodable, GameBoard, Phase, Team, Turn};
use crate::datastructures::game_board::UsefulGameBoard;
use crate::datastructures::Team::WHITE;
use crate::iterators::ChildTurnIterator;

pub struct MinimaxAgent {}

const GLOBAL_MAX_DEPTH: u16 = u16::MAX;
const ALPHA: f32 = f32::MIN;//1.1;
const BETA: f32 = 2.5;

#[derive(Debug, Hash, Eq, PartialEq)]
struct  TranspositionTableKey (u16, Turn);
type TranspositionTableValue = f32;
const TRANSPOSITION_ENTRY_SIZE: usize = size_of::<TranspositionTableKey>() + size_of::<TranspositionTableValue>() + 32;
const TRANSPOSITION_TABLE_CAPACITY: usize = 100_000_000; // 1,000,000,000 is roughly 2GB
type TranspositionTable = FnvHashMap<TranspositionTableKey, TranspositionTableValue>;

impl Agent for MinimaxAgent {
    fn get_next_move(phase: Phase, team: Team, board: GameBoard, history: Arc<Mutex<impl BoardHistory + 'static>>) -> Turn {
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
                let mut max_depth: u16 = 1;

                eprintln!("Initing transposition table: {}bytes * {}", TRANSPOSITION_ENTRY_SIZE, TRANSPOSITION_TABLE_CAPACITY);
                let mut transposition_table: TranspositionTable =
                    FnvHashMap::with_capacity_and_hasher(TRANSPOSITION_TABLE_CAPACITY, FnvBuildHasher::default());
                eprintln!("Transposition table initialized");

                // While we didn't receive a kill signal
                while !kill_signal_rx.try_recv().is_ok() {
                    Self::mini_max(phase, team, board, 0, max_depth, ALPHA, BETA, &mut transposition_table, Arc::clone(&history), Arc::clone(&runner_best_move));
                    eprintln!("Completed search with max depth {}", max_depth);
                    if max_depth < GLOBAL_MAX_DEPTH {
                        max_depth += 1;
                    } else {
                        eprintln!("Reached maximum depth");
                        break;
                    }
                }

                eprintln!("Minimax runner stopped");
            }).expect("Couldn't run minimax");

        // Wait until we used up most of the time
        thread::sleep(time::Duration::from_millis(990));

        let main_thread_best_move = Arc::clone(&current_best_move_mutex);
        let current_best_move = main_thread_best_move.lock()
            .expect("Expected there to be a best move, but there wasn't")
            .clone().expect("OH SHIT! I didn't find any best move. Now what?");
        // TODO: Choose any move if we didn't find a best move


        println!("{}", current_best_move.encode());
        eprintln!("Took {}ms", start_time.elapsed().unwrap().as_millis());

        // Tell the runner that its time to stop
        // Ignore failures, since the thread could stop earlier, if it reached max depth for example
        let _ = kill_signal_tx.send(());

        return current_best_move;
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
        history: Arc<Mutex<impl BoardHistory>>,
        current_best_move_mutex: Arc<Mutex<Option<Turn>>>
    ) -> f32 {
        let opponent = team_to_maximize.get_opponent();

        return if depth == max_depth || board.is_game_done() {
            evaluate_position(team_to_maximize, phase, board, depth, history)
        } else {
            let turns = ChildTurnIterator::new(
                phase,
                team_to_maximize,
                board
            ).sorted_unstable_by(|turn_a, turn_b| {
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

                let result = 3.0 - Self::mini_max(
                    new_phase,
                    opponent,
                    new_board,
                    depth + 1,
                    max_depth,
                    -beta,
                    m,
                    transposition_table,
                    history.clone(),
                    Arc::clone(&current_best_move_mutex)
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
                                Some(last_best) => eprintln!("Replacing current best move ({}) with new: {} (result: {result})", last_best.encode(), turn.encode()),
                                None => eprintln!("Setting initial best move to {} (result: {result})", turn.encode()),
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
        }
    }
}

#[test]
fn test_time_bounds() {
    let start = SystemTime::now();
    MinimaxAgent::get_next_move(
        Phase::MOVE,
        WHITE,
        GameBoard::decode(String::from("WWWBBBEEEEEEEEEEEEEEEEEE")),
        Arc::new(Mutex::new(BoardHistoryMap::default()))
    );
    let duration = SystemTime::now().duration_since(start).expect("Time went backwards");

    println!("Took {}ms", duration.as_millis());

    assert!(duration.as_millis() < 1000);
}