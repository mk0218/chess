use std::cmp::{max, min};

#[derive(Debug, PartialEq)]
pub enum ChessError {
    InvalidSquare,
    NotAMove,
    MoveFromEmpty,
    MoveFromEnemyPiece,
    IllegalMove,
    MoveToTeamPiece,
}

pub enum Team {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SquareState {
    Empty,
    White(Piece),
    Black(Piece),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(PartialEq)]
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

    pub fn move_piece(
        &mut self,
        turn: Team,
        from: &str,
        to: &str
    ) -> Result<Option<Piece>, ChessError>{
        use SquareState::*;

        let (r1, c1) = Board::convert_square_number(from)?;
        let (r2, c2) = Board::convert_square_number(to)?;

        self.is_legal_move(turn, r1, c1, r2, c2)?;

        self.0[r2][c2] = self.0[r1][c1];
        let killed = if let White(p) | Black(p) = self.0[r1][c1] {
            Some(p)
        } else {
            None
        };
        self.0[r1][c1] = Empty;

        Ok(killed)
    }

    /// Confirms that (r1, c1) -> (r2, c2) is a legal move
    /// Returns a corresponding Err(ChessError) if it is not a legal move.
    fn is_legal_move(
        &self,
        turn: Team,
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize,
    ) -> Result<(), ChessError> {
        use Piece::*;
        use SquareState::*;

        self.is_not_enemy(&turn, r1, c1)?;
        self.is_not_team(&turn, r2, c2)?;

        let dr = max(r1, r2) - min(r1, r2);
        let dc = max(c1, c2) - min(c1, c2);

        self.is_a_move(dr, dc)?;

        match self.0[r1][c1] {
            White(Pawn) => {
                match (r2 - r1, dc, self.0[r2][c2]) {
                    (1, 0, Empty) |
                    (1, 1, Black(_)) => Ok(()),
                    _ => Err(ChessError::IllegalMove),
                }
            }
            Black(Pawn) => {
                match (r1 - r2, dc, self.0[r2][c2]) {
                    (1, 0, Empty) |
                    (1, 1, White(_)) => Ok(()),
                    _ => Err(ChessError::IllegalMove),
                }
            }
            White(Rook) | Black(Rook) => {
                if dr == 0 || dc == 0 {
                    Ok(())
                } else {
                    Err(ChessError::IllegalMove)
                }
            }
            White(Knight) | Black(Knight) => {
                if (dr == 1 && dc == 2) || (dr == 2 && dc == 1) {
                    Ok(())
                } else {
                    Err(ChessError::IllegalMove)
                }
            }
            White(Bishop) | Black(Bishop) => {
                if dr == dc {
                    Ok(())
                } else {
                    Err(ChessError::IllegalMove)
                }
            }
            White(Queen) | Black(Queen) => {
                if dr == 0 || dc == 0 || dr == dc {
                    Ok(())
                } else {
                    Err(ChessError::IllegalMove)
                }
            }
            White(King) | Black(King) => {
                if dr <= 1 && dc <= 1 {
                    Ok(())
                } else {
                    Err(ChessError::IllegalMove)
                }
            }
            Empty => Err(ChessError::MoveFromEmpty)
        }
    }

    /// Returns Err when the given square (should be `from`) is empty.
    fn is_not_enemy(&self, turn: &Team, row: usize, col: usize) -> Result<(), ChessError> {
        match (turn, self.0[row][col]) {
            (Team::White, SquareState::Black(_)) |
            (Team::Black, SquareState::White(_)) => {
                Err(ChessError::MoveFromEnemyPiece)
            }
            _ => Ok(())
        }
    }

    /// Returns Err when the given square (should be `to`) is occupied by a team piece.
    fn is_not_team(&self, turn: &Team, row: usize, col: usize) -> Result<(), ChessError> {
        match (turn, self.0[row][col]) {
            (Team::White, SquareState::White(_)) |
            (Team::Black, SquareState::Black(_)) => {
                Err(ChessError::MoveToTeamPiece)
            }
            _ => Ok(())
        }
    }

    /// Returns Err when both dr and dc is 0. (That is, distance is 0.)
    fn is_a_move(&self, dr: usize, dc: usize) -> Result<(), ChessError> {
        if dr == 0 && dc == 0 {
            Err(ChessError::NotAMove)
        } else {
            Ok(())
        }
    }

    /// Converts the given square number as str into array indices.
    /// Returns Err when the given str is invalid or an index is invalid.
    fn convert_square_number(number: &str) -> Result<(usize, usize), ChessError> {
        let number = number.trim();
        
        if number.len() == 2 {
            let mut num = number.bytes();
            let row = num.next().unwrap();
            let col = num.next().unwrap();
            match (row, col) {
                (r @ b'1'..=b'8', c @ b'a'..=b'h') => {
                    Ok(((r - b'1').into(), (c - b'a').into()))
                }
                (r @ b'1'..=b'8', c @ b'A'..=b'H') => {
                    Ok(((r - b'1').into(), (c - b'A').into()))
                }
                _ => Err(ChessError::InvalidSquare)
            }
        } else {
            Err(ChessError::InvalidSquare)
        }
    }
}

#[cfg(test)]
mod test_board {
    use super::*;
    use Piece::*;
    use SquareState::*;

    #[test]
    fn test_convert_square_number_ok() {
        let result = Board::convert_square_number(" 1a ");
        assert_eq!(result, Ok((0, 0)));

        let result = Board::convert_square_number(" 8H ");
        assert_eq!(result, Ok((7, 7)));
    }

    #[test]
    fn test_convert_square_number_err() {
        let result = Board::convert_square_number("1z");
        assert_eq!(result, Err(ChessError::InvalidSquare));
        
        let result = Board::convert_square_number("a1");
        assert_eq!(result, Err(ChessError::InvalidSquare));
    }

    #[test]
    fn test_pawn_to_empty() {
        let mut board = Board::new();
        let _ = board.move_piece(Team::White, "2c", "3c");

        let expected = Board([
            [White(Rook), White(Knight), White(Bishop), White(King), White(Queen), White(Bishop), White(Knight), White(Rook)],
            [White(Pawn), White(Pawn), Empty, White(Pawn), White(Pawn), White(Pawn), White(Pawn), White(Pawn)],
            [Empty, Empty, White(Pawn), Empty, Empty, Empty, Empty, Empty],
            [Empty; 8],
            [Empty; 8],
            [Empty; 8],
            [Black(Pawn); 8],
            [Black(Rook), Black(Knight), Black(Bishop), Black(King), Black(Queen), Black(Bishop), Black(Knight), Black(Rook)], 
        ]);

        assert_eq!(board, expected);
    }

    #[test]
    fn test_is_legal_move() {
        let mut board = Board::new();
        assert_eq!(board.is_legal_move(Team::White, 1, 1, 2, 1), Ok(()));
        assert_eq!(board.is_legal_move(Team::Black, 6, 7, 5, 7), Ok(()));

        board.0[4][3] = SquareState::Black(Piece::Pawn);    // '5d'
        board.0[6][3] = SquareState::Empty;                 // '7d'
        board.0[3][2] = SquareState::White(Piece::Pawn);    // '4c'
        board.0[1][2] = SquareState::Empty;                 // '2c'

        assert_eq!(board.is_legal_move(Team::White, 3, 2, 4, 3), Ok(()));
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

impl std::fmt::Debug for Board {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "\n{}", self)
    }
}