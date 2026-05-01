/*
Bot name : ByteSlayer
Description : ByteSlayer is a chess bot, can be used in Discord.
Author : DOXI-dev
License : GPLv3
*/

// ----- FILES ----
mod uci;
mod evaluation;
mod engine;

// ----- MAIN -----
fn main() {
    uci::uci();
}
