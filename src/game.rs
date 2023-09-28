use crate::board::Board;

pub struct Game(Board);

impl Game {
    pub fn run() {
        let mut board = Board::new();
        println!("{board}");
    }
}