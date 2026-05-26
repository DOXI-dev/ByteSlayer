/*
This is the transposition tables file.
*/

// ----- LIBRARIES -----
use chess::ChessMove;

// ----- TRANSPOSITION -----
// Hash flags enum
#[derive(Copy, Clone, PartialEq)]
pub enum HashFlag {
    Exact,
    UpperBound,
    LowerBound,
}

// Entry structure
#[derive(Copy, Clone)]
pub struct TTEntry {
    pub key: u64,
    pub depth: u8,
    pub score: i32,
    pub flag: HashFlag,
    pub best_move: Option<ChessMove>,
}

// Global structure
pub struct TranspositionTable {
    pub table: Vec<TTEntry>,
}

// TranspositionTable implementation
impl TranspositionTable {
    // New function
    pub fn new(size: usize) -> TranspositionTable {
        TranspositionTable {
            table: vec![
                TTEntry {
                    key: 0,
                    depth: 0,
                    score: 0,
                    flag: HashFlag::Exact,
                    best_move: None
                };
                size
            ],
        }
    }

    // Position lookup function
    pub fn lookup(&self, key: u64) -> Option<TTEntry> {
        let index = key as usize % self.table.len();
        let entry = self.table[index];
        if entry.key == key { Some(entry) } else { None }
    }

    // Position storing function
    pub fn store(
        &mut self,
        key: u64,
        depth: u8,
        score: i32,
        flag: HashFlag,
        best_move: Option<ChessMove>,
    ) {
        let index = key as usize % self.table.len();
        let entry = self.table[index];
        if entry.key == 0 || depth >= entry.depth {
            self.table[index] = TTEntry {
                key,
                depth,
                score,
                flag,
                best_move,
            };
        }
    }
}
