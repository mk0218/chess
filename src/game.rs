use crate::board::Board;

pub struct Game(Board);

impl Game {
    pub fn run() {
        let board = Board::new();
        println!("{board}");
    }
}