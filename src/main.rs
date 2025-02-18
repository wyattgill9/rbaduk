use engine::{
    board::Board,
    board::Stone,
};

fn main() {
    let mut board = Board::new();
    println!("{board}");

    // Put pieces on the board with a repeating ".BW" pattern to check that
    // index computations are correct.
    let mut iter = [Stone::Empty, Stone::Black, Stone::White]
        .into_iter()
        .cycle();
    for row in 0..19 {
        for col in 0..19 {
            board.set(row, col, iter.next().unwrap());
        }
    }
    println!("{board}");
}