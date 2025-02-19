use engine::{board::Board, board::Stone, eval};

fn main() {
    let mut board = Board::new();

    println!("{}", board);

    loop {
        let user_input = std::io::stdin().lines().next().unwrap().unwrap();

        if user_input == "exit" {
            break;
        }

        let (row, col) = user_input.split_once(',').unwrap();
        let row = row.parse::<usize>().unwrap();
        let col = col.parse::<usize>().unwrap();

        eval::apply_move(&mut board, row, col, Stone::White);

        println!("{}", board);
    }
}
