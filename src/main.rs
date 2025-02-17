use engine::board;

fn main() {
    let board = board::Board::new();
    board.load_board();
    board.get_board_mem();
}
