use std::fmt::Display;

#[derive(Clone, Copy)]
enum Point {
    EMPTY,
    WHITE,
    BLACK,
}

impl Default for Point {
    fn default() -> Self {
        Point::EMPTY
    }
}

/// A 6 per side square playing board, containing 36 points.
/// These points either contain a white, black, or no piece.
struct Board {
    points: [Point; 36],
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            points: [Point::EMPTY; 36],
        };
        for i in (0..36).step_by(2) {
            board.points[i] = Point::BLACK;
            board.points[i + 1] = Point::WHITE;
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
                            Point::BLACK => 'B',
                            Point::WHITE => 'W',
                            Point::EMPTY => ' ',
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
    fn piece(&self, row: usize, col: usize) -> Option<Point> {
        if row > 5 || col > 5 {
            return None;
        }

        Some(self.points[row * 6 + col])
    }

    fn possible_moves(&self, row: usize, col: usize) -> Option<Vec<(usize, usize)>> {
        let moves = vec![];

        for horizontal in -1..2 {
            for vertical in -1..2 {
                if (horizontal != 0 && vertical != 0) || (horizontal == 0 && vertical == 0) {
                    continue;
                }
            }
        }

        Some(moves)
    }
}

fn main() {
    let board = Board::default();

    println!("{}", board);
}
