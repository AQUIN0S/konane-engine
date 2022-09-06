use konane_engine::Board;

fn main() {
    let board = Board::default();

    println!(
        "Possible moves from (1, 2): {:?}",
        board.possible_moves(1, 2).unwrap()
    );
}
