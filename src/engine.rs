/*
This is the chess engine that will select the best move to play.
*/

// ----- LIBRARIES -----
use crate::evaluation;
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
fn sort_moves(board: &Board, movegen: MoveGen) -> Vec<ChessMove> {
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
fn quiescence(board: &Board, mut alpha: i32, beta: i32) -> i32 {
    let static_eval = evaluation::evaluation(board);

    if static_eval >= beta {
        return beta;
    }
    if static_eval > alpha {
        alpha = static_eval;
    }
    let mut movegen = MoveGen::new_legal(board);
    movegen.set_iterator_mask(*board.color_combined(!board.side_to_move()));

    let sorted_moves = sort_moves(board, movegen);

    for m in sorted_moves {
        let new_board = board.make_move_new(m);
        let score = -quiescence(&new_board, -beta, -alpha);

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
fn negamax(board: &Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    match board.status() {
        BoardStatus::Checkmate => return -20000 + depth,
        BoardStatus::Stalemate => return 0,
        BoardStatus::Ongoing => {}
    }

    if depth == 0 {
        return quiescence(board, alpha, beta);
    }

    let movegen = MoveGen::new_legal(board);
    let new_movegen = sort_moves(board, movegen);

    let mut value: i32 = -INFINITY;

    for m in new_movegen {
        let new_board = board.make_move_new(m);

        let score = -negamax(&new_board, depth - 1, -beta, -alpha);

        value = value.max(score);
        alpha = alpha.max(value);

        if alpha >= beta {
            break;
        }
    }
    value
}

// Function to choose a move
pub fn choose_move(depth: i32, board: &Board) -> Option<ChessMove> {
    let mut best_move: Option<ChessMove> = None;
    let mut alpha = -INFINITY;
    let beta = INFINITY;

    let movegen = MoveGen::new_legal(board);
    let new_movegen = sort_moves(board, movegen);

    for m in new_movegen {
        let new_board = board.make_move_new(m);

        let value = -negamax(&new_board, depth - 1, -beta, -alpha);

        if value > alpha {
            alpha = value;
            best_move = Some(m);
        }
    }
    best_move
}
