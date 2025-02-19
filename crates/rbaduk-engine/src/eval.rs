use crate::board::{Board, Stone};
// use crate::pos::Position;

pub fn calc_liberties(board: &Board, row: usize, col: usize) -> u8 {
    let mut liberties: u8 = 0;
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let stone = board.get(row, col);
    if stone == Stone::Empty {
        return 0;
    }

    for &(dr, dc) in &directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0 && new_row < 19 && new_col >= 0 && new_col < 19 {
            if board.get(new_row as usize, new_col as usize) == Stone::Empty {
                liberties += 1;
            }
        }
    }

    liberties
}

pub fn get_captures(board: &Board, row: usize, col: usize, color: Stone) -> Vec<(usize, usize)> {
    let mut captures: Vec<(usize, usize)> = Vec::new();
    let opposite_color = match color {
        Stone::Black => Stone::White,
        Stone::White => Stone::Black,
        Stone::Empty => return captures,
    };
    let pos = (row, col);

    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dr, dc) in &directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0 && new_row < 19 && new_col >= 0 && new_col < 19 {
            if board.get(new_row as usize, new_col as usize) == opposite_color {
                if 
                captures.push((new_row as usize, new_col as usize));
            }
        }
    }

    captures
}

pub fn apply_move(board: &mut Board, row: usize, col: usize, color: Stone) {
    if board.get(row, col) != Stone::Empty {
        println!("Invalid move: ({row}, {col}), already occupied by other stone.");
        return;
    }

    let captures = get_captures(board, row, col, color);

    if captures.is_empty() {
        board.place_stones(row, col, color);
        return;
    }

    println!("{:?} captures {:?}.", color, captures);
    unsafe {
        board.place_stones_unchecked(row, col, color);
        // SAFETY: Both `row` and `col` are less than 19.
        for &(r, c) in &captures {
            board.place_stones_unchecked(r, c, Stone::Empty);
        }
    }
}
