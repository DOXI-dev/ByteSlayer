/*
This is the file for the UCI communication protocol.
*/

// ----- LIBRARIES -----
use crate::engine::choose_move;
use chess::{Board, ChessMove};
use std::io::{self, BufRead};
use std::str::FromStr;
use std::time::{Duration, Instant};

// ----- UCI ------
// Position handling function for the "position [startpos|fen <fen>] moves <m1> <m2> ..." command
fn handle_position(board: &mut Board, command: Vec<&str>) {
    let moves_idx = command.iter().position(|&t| t == "moves");

    match command.get(1) {
        Some(&"startpos") => *board = Board::default(),
        Some(&"fen") => {
            let end_fen = moves_idx.unwrap_or(command.len());
            if end_fen > 2 {
                let fen = command[2..end_fen].join(" ");
                if let Ok(new_board) = Board::from_str(&fen) {
                    *board = new_board;
                }
            }
        }
        _ => return,
    }

    if let Some(idx) = moves_idx {
        for move_str in command.iter().skip(idx + 1) {
            if let Ok(m) = ChessMove::from_str(move_str) {
                *board = board.make_move_new(m);
            }
        }
    }
}

// go function for "go wtime <x> btime <x>" command
fn go(board: &mut Board, command: Vec<&str>) {
    let mut wtime = None;
    let mut btime = None;

    for i in 0..command.len() {
        match command[i] {
            "wtime" => wtime = command.get(i + 1).and_then(|s| s.parse::<u32>().ok()),
            "btime" => btime = command.get(i + 1).and_then(|s| s.parse::<u32>().ok()),
            _ => {}
        }
    }

    let my_time = if board.side_to_move() == chess::Color::White {
        wtime
    } else {
        btime
    };

    let time_limit_ms = my_time.map(|t| t / 40).unwrap_or(2000);
    let time_limit = Duration::from_millis(time_limit_ms as u64);

    let start = Instant::now();

    let mut best_mv = None;

    for depth in 1..100 {
        let elapsed = start.elapsed();

        if elapsed > time_limit.mul_f32(0.6) && depth > 1 {
            break;
        }

        if let Some(new_mv) = choose_move(depth, board) {
            best_mv = Some(new_mv);
        }

        if start.elapsed() >= time_limit {
            break;
        }
    }

    if let Some(m) = best_mv {
        println!("bestmove {}", m);
    } else {
        let mut move_it = chess::MoveGen::new_legal(board);
        if let Some(m) = move_it.next() {
            println!("bestmove {}", m);
        }
    }
}

// Uci IO function
pub fn uci() {
    let stdin = io::stdin();
    let mut board = Board::default();

    for line in stdin.lock().lines() {
        let raw_line = line.unwrap();

        let commands: Vec<&str> = raw_line.split_whitespace().collect();

        match commands.get(0) {
            Some(&"uci") => {
                println!("id name ByteSlayer");
                println!("id author DOXI-dev");
                println!("option name Move Overhead type spin default 10 min 0 max 5000");
                println!("uciok");
            }
            Some(&"isready") => println!("readyok"),
            Some(&"setoption") => {}
            Some(&"position") => {
                handle_position(&mut board, commands);
            }
            Some(&"go") => {
                go(&mut board, commands);
            }
            Some(&"quit") => break,
            _ => {}
        }
    }
}
