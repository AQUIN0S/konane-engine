use std::fmt::Display;

use super::Piece;

/// A 6 per side square playing board, containing 36 points.
/// These points either contain a white, black, or no piece.
pub struct Board {
    points: [Piece; 36],
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::create_empty();
        for i in (0..36).step_by(2) {
            board.points[i] = Piece::BLACK;
            board.points[i + 1] = Piece::WHITE;
        }

        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..6 {
            writeln!(
                f,
                "{}",
                self.points[row * 6..row * 6 + 6]
                    .iter()
                    .map(|point| {
                        let point_char = String::from(match point {
                            Piece::BLACK => 'B',
                            Piece::WHITE => 'W',
                            Piece::EMPTY => ' ',
                        }) + " ";

                        point_char
                    })
                    .collect::<String>()
            )?;
        }

        Ok(())
    }
}

impl Board {
    /// Create an empty board
    ///
    /// # Example
    ///
    /// ```rust
    /// use konane_engine::{Board, Piece};
    ///
    /// let board = Board::create_empty();
    ///
    /// for row in 0..5 {
    ///     for col in 0..5 {
    ///         assert_eq!(board.get_piece(row, col), Some(Piece::EMPTY));
    ///     }
    /// }
    /// ```
    pub fn create_empty() -> Board {
        Board {
            points: [Piece::EMPTY; 36],
        }
    }

    /// Get the piece at a given position
    ///
    /// # Example
    ///
    /// ```rust
    /// use konane_engine::{Board, Piece};
    ///
    /// let board = Board::default();
    /// assert_eq!(board.get_piece(0, 0), Some(Piece::BLACK));
    ///
    /// // Off board range
    /// assert_eq!(board.get_piece(6, 1), None);
    /// ```
    pub fn get_piece(&self, row: usize, col: usize) -> Option<Piece> {
        if row > 5 || col > 5 {
            return None;
        }

        Some(self.points[row * 6 + col])
    }

    /// Set the piece at a given location to a given piece type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konane_engine::{Board, Piece};
    ///
    /// let mut board = Board::create_empty();
    ///
    /// let captured_piece = board.set_piece(0, 1, Piece::WHITE).unwrap();
    /// assert_eq!(captured_piece, Piece::EMPTY);
    /// assert_eq!(board.get_piece(0, 1), Some(Piece::WHITE));
    ///
    /// let captured_piece = board.set_piece(0, 1, Piece::BLACK).unwrap();
    /// assert_eq!(captured_piece, Piece::WHITE);
    /// assert_eq!(board.get_piece(0, 1), Some(Piece::BLACK));
    /// ```
    pub fn set_piece(&mut self, row: usize, col: usize, piece_type: Piece) -> Option<Piece> {
        match self.get_piece(row, col) {
            Some(piece) => {
                self.points[row * 6 + col] = piece_type;
                Some(piece)
            }
            None => None,
        }
    }

    /// Return a list of the possible moves that can be made from a given position on the board.
    /// An assumption is made that only enemy pieces will be placed adjacent to a piece.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konane_engine::{Board, Piece};
    ///
    /// let mut board = Board::create_empty();
    ///
    /// let _ = board.set_piece(0, 0, Piece::BLACK);
    /// let _ = board.set_piece(0, 1, Piece::WHITE);
    /// let _ = board.set_piece(0, 2, Piece::BLACK);
    /// let _ = board.set_piece(0, 4, Piece::BLACK);
    ///
    /// let possible_moves = board.possible_moves(0, 1).unwrap();
    /// assert_eq!(possible_moves, vec![(0, 3), (0, 5)]);
    /// ```
    pub fn possible_moves(&self, row: usize, col: usize) -> Option<Vec<(usize, usize)>> {
        if let None = self.get_piece(row, col) {
            return None;
        }

        let mut moves = vec![];

        // Vertical
        for i in [-1, 1] {
            for jump in (0..).step_by(2) {
                let from_row = row as isize + i * jump;

                // If moving takes the piece out of bounds, we instantly continue.
                if from_row + i * 2 < 0 || from_row + i * 2 > 5 {
                    break;
                }

                let enemy_row = (from_row + i) as usize;
                let to_row = (from_row + i * 2) as usize;

                if let Some(enemy) = self.get_piece(enemy_row, col) {
                    match enemy {
                        Piece::EMPTY => break,
                        _ => {
                            if let Some(Piece::EMPTY) = self.get_piece(to_row, col) {
                                moves.push((to_row, col));
                            }
                        }
                    }
                }
            }
        }

        // Horizontal
        for i in [-1, 1] {
            for jump in (0..).step_by(2) {
                let from_col = col as isize + i * jump;

                // If moving takes the piece out of bounds, we instantly continue.
                if from_col + i * 2 < 0 || from_col + i * 2 > 5 {
                    break;
                }

                let enemy_col = (from_col + i) as usize;
                let to_col = (from_col + i * 2) as usize;

                if let Some(enemy) = self.get_piece(row, enemy_col) {
                    match enemy {
                        Piece::EMPTY => break,
                        _ => {
                            if let Some(Piece::EMPTY) = self.get_piece(row, to_col) {
                                moves.push((row, to_col));
                            }
                        }
                    }
                }
            }
        }

        Some(moves)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Piece};

    #[test]
    fn can_jump_once() {
        let mut board = Board::create_empty();

        let _ = board.set_piece(0, 0, Piece::BLACK);

        assert_eq!(board.possible_moves(0, 0), Some(vec![]));

        let _ = board.set_piece(3, 3, Piece::BLACK);
        let _ = board.set_piece(3, 4, Piece::WHITE);
        let _ = board.set_piece(4, 3, Piece::WHITE);

        let mut possible_moves = board.possible_moves(3, 3).unwrap();
        possible_moves.sort();

        let mut ideal = vec![(3, 5), (5, 3)];
        ideal.sort();

        assert_eq!(possible_moves, ideal);
    }
}
