/*
This is the evaluation file for the chess engine. This code evaluates a position and assigns it a score.
*/

// ----- LIBRARIES -----
use chess::{Board, Color, Piece};

// ----- MODEL -----
// Evaluation function
pub fn evaluation(board: &Board) -> i32 {
    let mut score = 0;

    let pieces = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
        Piece::King,
    ];

    for piece in pieces {
        let value = match piece {
            Piece::Pawn => 100,
            Piece::Knight => 320,
            Piece::Bishop => 330,
            Piece::Rook => 500,
            Piece::Queen => 900,
            Piece::King => 20000,
        };

        let white_bb = board.pieces(piece) & board.color_combined(Color::White);
        score += (white_bb.popcnt() as i32) * value;

        let black_bb = board.pieces(piece) & board.color_combined(Color::Black);
        score -= (black_bb.popcnt() as i32) * value;
    }

    let perspective = if board.side_to_move() == Color::White {
        1
    } else {
        -1
    };

    score * perspective
}
