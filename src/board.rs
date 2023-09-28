#[derive(Clone, Copy)]
enum SquareState {
    Empty,
    White(Piece),
    Black(Piece),
}

#[derive(Clone, Copy)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Board([[SquareState; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        use Piece::*;
        use SquareState::*;

        let mut board = [[Empty; 8]; 8];

        let fst_row = [
            Rook,
            Knight,
            Bishop,
            King,
            Queen,
            Bishop,
            Knight,
            Rook,
        ];

        for col in 0..=7 {
            board[0][col] = White(fst_row[col]);
            board[7][col] = Black(fst_row[col]);
            board[1][col] = White(Pawn);
            board[6][col] = Black(Pawn);
        }

        Self(board)
    }
}

impl std::fmt::Display for SquareState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SquareState::*;

        match self {
            Empty => write!(formatter, "   "),
            White(p) => write!(formatter, "W{}", p),
            Black(p) => write!(formatter, "B{}", p)
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Piece::*;

        write!(formatter, "{:>2}", match self {
            Pawn => "P",
            Rook => "R",
            Knight => "KN",
            Bishop => "B",
            Queen => "Q",
            King => "K",
        })
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in (0..8).rev() {
            write!(formatter, "  +---+---+---+---+---+---+---+---+\n")?;
            write!(formatter, "{} ", row + 1)?;
            for col in 0..8 {
                write!(formatter, "|{}", self.0[row][col])?;
            }
            write!(formatter, "|\n")?;
        }
        write!(formatter, "  +---+---+---+---+---+---+---+---+\n")?;
        write!(formatter, "    a   b   c   d   e   f   g   h\n")
    }
}