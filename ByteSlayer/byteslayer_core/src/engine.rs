/*
This is the chess engine that will select the best move to play.
*/

// ----- LIBRARIES -----
use crate::evaluation;
use crate::transposition::{HashFlag, TranspositionTable};
use chess::{BitBoard, Board, BoardStatus, ChessMove, MoveGen, Piece};

// ----- MODEL -----
// Function to avoid duplicating piece values
fn get_piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 20000,
    }
}

// Move sorting for the negamax function
fn sort_moves(board: &Board, movegen: MoveGen, best_move: Option<ChessMove>) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = movegen.collect();
    let mut targets = *board.color_combined(!board.side_to_move());

    if let Some(ep_sq) = board.en_passant() {
        targets |= BitBoard::from_square(ep_sq);
    }

    moves.sort_by_key(|m| {
        let mut score = 0;
        let destination = m.get_dest();
        let source = m.get_source();
        let dest_mask = BitBoard::from_square(destination);

        if Some(*m) == best_move {
            score += 100000
        }

        if (targets & dest_mask).popcnt() > 0 {
            if let Some(capturing_piece) = board.piece_on(source) {
                if let Some(captured_piece) = board.piece_on(destination) {
                    let captured_value = get_piece_value(captured_piece);
                    let capturing_value = get_piece_value(capturing_piece);

                    score += 1000 + (captured_value - capturing_value);
                } else {
                    score += 1000 + 100;
                }
            }
        }
        -score
    });
    moves
}

// Quiescence search function
fn quiescence(board: &Board, mut alpha: i32, beta: i32, genome: &evaluation::Genome) -> i32 {
    let stand_pat = evaluation::evaluation(board, genome);

    if stand_pat >= beta {
        return beta;
    }

    alpha = alpha.max(stand_pat);

    let mut movegen = MoveGen::new_legal(board);
    movegen.set_iterator_mask(*board.color_combined(!board.side_to_move()));

    let sorted_moves = sort_moves(board, movegen, None);

    for m in sorted_moves {
        let new_board = board.make_move_new(m);
        let score = -quiescence(&new_board, -beta, -alpha, genome);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

// Constant for "infinite numbers"
const INFINITY: i32 = 2_000_000;

// Negamax and alpha-beta pruning function
fn negamax(
    board: &Board,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    tt: &mut TranspositionTable,
    genome: &evaluation::Genome,
) -> i32 {
    let alpha_orig = alpha;
    let mut best_move_found: Option<ChessMove> = None;

    match board.status() {
        BoardStatus::Checkmate => return -20000 + depth,
        BoardStatus::Stalemate => return 0,
        BoardStatus::Ongoing => {}
    }

    let key = board.get_hash();

    let mut tt_best_move = None;

    if let Some(entry) = tt.lookup(key) {
        tt_best_move = entry.best_move;

        if entry.depth >= depth as u8 {
            if entry.flag == HashFlag::Exact
                || entry.flag == HashFlag::LowerBound && entry.score >= beta
                || entry.flag == HashFlag::UpperBound && entry.score <= alpha
            {
                return entry.score;
            }
            tt_best_move = entry.best_move;
        }
    }

    if depth == 0 {
        return quiescence(board, alpha, beta, genome);
    }

    let movegen = MoveGen::new_legal(board);
    let new_movegen = sort_moves(board, movegen, tt_best_move);

    let mut value: i32 = -INFINITY;

    for m in new_movegen {
        let new_board = board.make_move_new(m);

        let score = -negamax(&new_board, depth - 1, -beta, -alpha, tt, genome);

        if score > value {
            value = score;
            best_move_found = Some(m);
        }

        alpha = alpha.max(value);

        if alpha >= beta {
            break;
        }
    }

    let flag = if value <= alpha_orig {
        HashFlag::UpperBound
    } else if value >= beta {
        HashFlag::LowerBound
    } else {
        HashFlag::Exact
    };

    tt.store(key, depth as u8, value, flag, best_move_found);

    value
}

// Function to choose a move
pub fn choose_move(
    depth: i32,
    board: &Board,
    previous_best_move: Option<ChessMove>,
    tt: &mut TranspositionTable,
    genome: &evaluation::Genome,
) -> Option<ChessMove> {
    let mut best_move: Option<ChessMove> = None;
    let mut alpha = -INFINITY;
    let beta = INFINITY;

    let movegen = MoveGen::new_legal(board);
    let new_movegen = sort_moves(board, movegen, previous_best_move);

    for m in new_movegen {
        let new_board = board.make_move_new(m);

        let value = -negamax(&new_board, depth - 1, -beta, -alpha, tt, genome);

        if value > alpha {
            alpha = value;
            best_move = Some(m);
        }
    }
    best_move
}
