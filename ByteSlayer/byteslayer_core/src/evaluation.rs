/*
This is the evaluation file for the chess engine. This code evaluates a position and assigns it a score.
*/

// ----- LIBRARIES -----
use chess::{Board, Color, Piece};

// ----- MODEL -----
// Genome
#[derive(Clone, Debug)]
pub struct Genome {
    pub p_pst_mg: [i32; 64],
    pub p_pst_eg: [i32; 64],
    pub n_pst_mg: [i32; 64],
    pub n_pst_eg: [i32; 64],
    pub b_pst_mg: [i32; 64],
    pub b_pst_eg: [i32; 64],
    pub r_pst_mg: [i32; 64],
    pub r_pst_eg: [i32; 64],
    pub q_pst_mg: [i32; 64],
    pub q_pst_eg: [i32; 64],
    pub k_pst_mg: [i32; 64],
    pub k_pst_eg: [i32; 64],
}

// Evaluation function
pub fn evaluation(board: &Board, genome: &Genome) -> i32 {
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
            Piece::Pawn => &genome.p_pst_mg,
            Piece::Knight => &genome.n_pst_mg,
            Piece::Bishop => &genome.b_pst_mg,
            Piece::Rook => &genome.r_pst_mg,
            Piece::Queen => &genome.q_pst_mg,
            Piece::King => &genome.k_pst_mg,
        };

        let pst_eg = match piece {
            Piece::Pawn => &genome.p_pst_eg,
            Piece::Knight => &genome.n_pst_eg,
            Piece::Bishop => &genome.b_pst_eg,
            Piece::Rook => &genome.r_pst_eg,
            Piece::Queen => &genome.q_pst_eg,
            Piece::King => &genome.k_pst_eg,
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
    let mut white_relative_score = ((score_mg * phase) + (score_eg * (24 - phase))) / 24;

    white_relative_score += evaluate_pawns_bonus(board, Color::White);
    white_relative_score -= evaluate_pawns_bonus(board, Color::Black);

    let perspective = if board.side_to_move() == Color::White {
        1
    } else {
        -1
    };

    white_relative_score * perspective
}

// Fonction to get the default genome
pub fn get_default_genome() -> Genome {
    Genome {
        p_pst_mg: crate::pst::P_PST_MG,
        p_pst_eg: crate::pst::P_PST_EG,
        n_pst_mg: crate::pst::N_PST_MG,
        n_pst_eg: crate::pst::N_PST_EG,
        b_pst_mg: crate::pst::B_PST_MG,
        b_pst_eg: crate::pst::B_PST_EG,
        r_pst_mg: crate::pst::R_PST_MG,
        r_pst_eg: crate::pst::R_PST_EG,
        q_pst_mg: crate::pst::Q_PST_MG,
        q_pst_eg: crate::pst::Q_PST_EG,
        k_pst_mg: crate::pst::K_PST_MG,
        k_pst_eg: crate::pst::K_PST_EG,
    }
}

// Give an exponential bonus to pawns who reach the final rank
fn evaluate_pawns_bonus(board: &Board, color: Color) -> i32 {
    let mut bonus = 0;

    let pawns = board.pieces(Piece::Pawn) & board.color_combined(color);

    for square in pawns {
        let rank = square.get_rank().to_index();

        if color == Color::White {
            if rank >= 4 {
                bonus += 10 * i32::pow(4, (rank - 4) as u32);
            }
        } else {
            if rank <= 3 {
                let distance_avancee = 3 - rank;
                bonus += 10 * i32::pow(4, distance_avancee as u32);
            }
        }
    }
    bonus
}
