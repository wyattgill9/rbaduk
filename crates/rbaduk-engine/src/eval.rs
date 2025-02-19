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
    let opponent_color = match color {
        Stone::Black => Stone::White,
        Stone::White => Stone::Black,
        Stone::Empty => return Vec::new(),
    };

    let mut visited = [[false; 19]; 19];
    let mut captures = Vec::new();

    for (dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let adj_r = row as isize + dr;
        let adj_c = col as isize + dc;

        if adj_r < 0 || adj_r >= 19 || adj_c < 0 || adj_c >= 19 {
            continue;
        }

        let adj_r = adj_r as usize;
        let adj_c = adj_c as usize;

        if board.get(adj_r, adj_c) != opponent_color || visited[adj_r][adj_c] {
            continue;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((adj_r, adj_c));
        visited[adj_r][adj_c] = true;
        let mut group = vec![(adj_r, adj_c)];

        while let Some((r, c)) = queue.pop_front() {
            for (dr_inner, dc_inner) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nr = r as isize + dr_inner;
                let nc = c as isize + dc_inner;

                if nr < 0 || nr >= 19 || nc < 0 || nc >= 19 {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;

                if !visited[nr][nc] && board.get(nr, nc) == opponent_color {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc));
                    group.push((nr, nc));
                }
            }
        }

        let mut has_liberty = false;
        'liberty_check: for &(r, c) in &group {
            for (dr_liberty, dc_liberty) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let lr = r as isize + dr_liberty;
                let lc = c as isize + dc_liberty;

                if lr < 0 || lr >= 19 || lc < 0 || lc >= 19 {
                    continue;
                }

                let lr = lr as usize;
                let lc = lc as usize;

                if lr == row && lc == col {
                    continue;
                }

                if board.get(lr, lc) == Stone::Empty {
                    has_liberty = true;
                    break 'liberty_check;
                }
            }
        }

        if !has_liberty {
            captures.extend(group);
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
