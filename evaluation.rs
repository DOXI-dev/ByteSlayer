/*
This is the evaluation file for the chess engine. This code evaluates a position and assigns it a score.
*/

// ----- LIBRARIES -----
use crate::pst;
use chess::{Board, Color, Piece};

// ----- MODEL -----
// Evaluation function
pub fn evaluation(board: &Board) -> i32 {
    let mut score_mg = 0;
    let mut score_eg = 0;
    let mut phase = 0;

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

        let piece_phase_val = match piece {
            Piece::Knight => 1,
            Piece::Bishop => 1,
            Piece::Rook => 2,
            Piece::Queen => 4,
            _ => 0,
        };

        let pst_mg = match piece {
            Piece::Pawn => &pst::P_PST_MG,
            Piece::Knight => &pst::N_PST_MG,
            Piece::Bishop => &pst::B_PST_MG,
            Piece::Rook => &pst::R_PST_MG,
            Piece::Queen => &pst::Q_PST_MG,
            Piece::King => &pst::K_PST_MG,
        };

        let pst_eg = match piece {
            Piece::Pawn => &pst::P_PST_EG,
            Piece::Knight => &pst::N_PST_EG,
            Piece::Bishop => &pst::B_PST_EG,
            Piece::Rook => &pst::R_PST_EG,
            Piece::Queen => &pst::Q_PST_EG,
            Piece::King => &pst::K_PST_EG,
        };

        let white_bb = board.pieces(piece) & board.color_combined(Color::White);
        for square in white_bb {
            let index = square.to_index();

            score_mg += pst_mg[index] + value;
            score_eg += pst_eg[index] + value;
            phase += piece_phase_val;
        }

        let black_bb = board.pieces(piece) & board.color_combined(Color::Black);
        for square in black_bb {
            let index = square.to_index() ^ 56;

            score_mg -= pst_mg[index] + value;
            score_eg -= pst_eg[index] + value;
            phase += piece_phase_val;
        }
    }

    let phase = phase.min(24);
    let white_relative_score = ((score_mg * phase) + (score_eg * (24 - phase))) / 24;

    let perspective = if board.side_to_move() == Color::White {
        1
    } else {
        -1
    };

    white_relative_score * perspective
}
