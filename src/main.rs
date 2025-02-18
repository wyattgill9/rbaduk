use engine::{
    board,
    board::Stone::{Black, White},
};

fn main() {
    let mut board = board::Board::new();

    board.place_stone((0, 0), Black);
    board.place_stone((1, 1), White);

    println!("{}", board);
    board.get_board_mem();
}
