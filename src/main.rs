use engine::{board::Board, board::Stone, eval};

fn main() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 1, 1, Stone::White);

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);
    eval::apply_move(&mut board, 2, 1, Stone::Black);

    println!("{}", board);

    eval::apply_move(&mut board, 1, 2, Stone::Black);

    println!("{}", board);
}
