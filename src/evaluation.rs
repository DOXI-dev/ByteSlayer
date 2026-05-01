/*
This is the evaluation file for the chess engine. This code evaluates a position and assigns it a score.
*/

// ----- LIBRARIES -----
use chess::{Board, Color, Piece, ALL_SQUARES};

// ----- MODEL -----
// Evaluation function
pub fn evaluation(board: &Board) -> i32 {
    let mut score = 0;

    for &square in &ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            let color = board.color_on(square).unwrap();

            let value = match piece {
                Piece::Pawn => 100,
                Piece::Knight => 320,
                Piece::Bishop => 330,
                Piece::Rook => 500,
                Piece::Queen => 900,
                Piece::King => 20000,
            };

            if color == Color::White {
                score += value;
            } else {
                score -= value;
            }
        }
    }

    let perspective = if board.side_to_move() == Color::White {
        1
    } else {
        -1
    };

    score * perspective
}
