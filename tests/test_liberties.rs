use engine::{board::Board, board::Stone, eval};

#[test]
fn test_single_stone_liberties() {
    let mut board = Board::new();
    board.place_stones(2, 2, Stone::Black);
    let liberties = eval::calc_liberties(&board, 2, 2);
    assert_eq!(liberties, 4);
}

#[test]
fn test_edge_stone_liberties() {
    let mut board = Board::new();
    board.place_stones(0, 3, Stone::Black);
    let liberties = eval::calc_liberties(&board, 0, 3);
    assert_eq!(liberties, 3);
}

#[test]
fn test_corner_stone_liberties() {
    let mut board = Board::new();
    board.place_stones(0, 0, Stone::Black);
    let liberties = eval::calc_liberties(&board, 0, 0);
    assert_eq!(liberties, 2);
}

#[test]
fn test_surrounded_stone_liberties() {
    let mut board = Board::new();
    board.place_stones(2, 2, Stone::Black);
    board.place_stones(1, 2, Stone::White);
    board.place_stones(3, 2, Stone::White);
    board.place_stones(2, 1, Stone::White);
    board.place_stones(2, 3, Stone::White);
    let liberties = eval::calc_liberties(&board, 2, 2);
    assert_eq!(liberties, 0);
}
