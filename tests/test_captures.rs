use engine::{
    board::Board,
    board::Stone,
    eval::{apply_move, get_captures},
};

#[test]
fn test_single_capture() {
    let mut board = Board::new();

    board.place_stones(0, 1, Stone::White);
    board.place_stones(1, 0, Stone::White);
    board.place_stones(1, 2, Stone::White);
    board.place_stones(2, 1, Stone::White);
    board.place_stones(1, 1, Stone::Black);

    assert_eq!(get_captures(&board, 1, 1, Stone::Black), vec![]);

    apply_move(&mut board, 1, 1, Stone::White);

    assert_eq!(board.get(1, 1), Stone::White);
}
