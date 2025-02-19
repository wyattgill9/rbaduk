use engine::{board::Board, board::Stone, eval};

#[test]
fn test_single_capture() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 1, 1, Stone::White);

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);
    eval::apply_move(&mut board, 2, 1, Stone::Black);

    assert_eq!(board.get(1, 2), Stone::Empty);

    eval::apply_move(&mut board, 1, 2, Stone::Black);

    assert_eq!(board.get(1, 1), Stone::Empty);
}

#[test]
fn test_no_capture() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 1, 1, Stone::White);

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);

    assert_eq!(board.get(1, 1), Stone::White);

    eval::apply_move(&mut board, 2, 1, Stone::Black);

    assert_eq!(board.get(1, 1), Stone::White);
}

#[test]
fn test_multiple_captures() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 1, 1, Stone::White);
    eval::apply_move(&mut board, 1, 2, Stone::White);
    eval::apply_move(&mut board, 2, 1, Stone::White);
    eval::apply_move(&mut board, 2, 2, Stone::White);

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 0, 2, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);
    eval::apply_move(&mut board, 2, 0, Stone::Black);
    eval::apply_move(&mut board, 3, 1, Stone::Black);
    eval::apply_move(&mut board, 3, 2, Stone::Black);
    eval::apply_move(&mut board, 1, 3, Stone::Black);
    eval::apply_move(&mut board, 2, 3, Stone::Black);

    eval::apply_move(&mut board, 1, 1, Stone::Black);

    assert_eq!(board.get(1, 1), Stone::Black);
    assert_eq!(board.get(1, 2), Stone::Empty);
    assert_eq!(board.get(2, 1), Stone::Empty);
    assert_eq!(board.get(2, 2), Stone::Empty);
}

#[test]
fn test_edge_capture() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 0, 0, Stone::White);
    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);

    eval::apply_move(&mut board, 1, 1, Stone::Black);

    assert_eq!(board.get(0, 0), Stone::Empty);
}

#[test]
fn test_corner_capture() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 0, 0, Stone::White);
    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);

    eval::apply_move(&mut board, 1, 1, Stone::Black);

    assert_eq!(board.get(0, 0), Stone::Empty);
}

#[test]
fn test_invalid_move() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 1, 1, Stone::White);

    eval::apply_move(&mut board, 1, 1, Stone::Black);

    assert_eq!(board.get(1, 1), Stone::White);
}

#[test]
fn test_self_capture() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);
    eval::apply_move(&mut board, 1, 2, Stone::Black);
    eval::apply_move(&mut board, 2, 1, Stone::Black);

    eval::apply_move(&mut board, 1, 1, Stone::White);

    assert_eq!(board.get(1, 1), Stone::Empty);
}

#[test]
fn test_complex_capture() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 1, 1, Stone::White);
    eval::apply_move(&mut board, 1, 2, Stone::White);
    eval::apply_move(&mut board, 2, 1, Stone::White);
    eval::apply_move(&mut board, 2, 2, Stone::White);

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 0, 2, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);
    eval::apply_move(&mut board, 2, 0, Stone::Black);
    eval::apply_move(&mut board, 3, 1, Stone::Black);
    eval::apply_move(&mut board, 3, 2, Stone::Black);
    eval::apply_move(&mut board, 1, 3, Stone::Black);
    eval::apply_move(&mut board, 2, 3, Stone::Black);

    eval::apply_move(&mut board, 1, 1, Stone::Black);

    assert_eq!(board.get(1, 1), Stone::Black);
    assert_eq!(board.get(1, 2), Stone::Empty);
    assert_eq!(board.get(2, 1), Stone::Empty);
    assert_eq!(board.get(2, 2), Stone::Empty);
}

#[test]
fn test_no_self_atari() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);
    eval::apply_move(&mut board, 1, 2, Stone::Black);
    eval::apply_move(&mut board, 2, 1, Stone::Black);

    eval::apply_move(&mut board, 1, 1, Stone::White);

    assert_eq!(board.get(1, 1), Stone::Empty);
}

#[test]
fn test_large_group_capture() {
    let mut board = Board::new();

    eval::apply_move(&mut board, 1, 1, Stone::White);
    eval::apply_move(&mut board, 1, 2, Stone::White);
    eval::apply_move(&mut board, 2, 1, Stone::White);
    eval::apply_move(&mut board, 2, 2, Stone::White);

    eval::apply_move(&mut board, 0, 1, Stone::Black);
    eval::apply_move(&mut board, 0, 2, Stone::Black);
    eval::apply_move(&mut board, 1, 0, Stone::Black);
    eval::apply_move(&mut board, 2, 0, Stone::Black);
    eval::apply_move(&mut board, 3, 1, Stone::Black);
    eval::apply_move(&mut board, 3, 2, Stone::Black);
    eval::apply_move(&mut board, 1, 3, Stone::Black);
    eval::apply_move(&mut board, 2, 3, Stone::Black);

    eval::apply_move(&mut board, 1, 1, Stone::Black);

    assert_eq!(board.get(1, 1), Stone::Black);
    assert_eq!(board.get(1, 2), Stone::Empty);
    assert_eq!(board.get(2, 1), Stone::Empty);
    assert_eq!(board.get(2, 2), Stone::Empty);
}
