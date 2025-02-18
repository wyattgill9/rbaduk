use core::mem;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Stone {
    #[default]
    Empty,
    Black,
    White,
}

impl core::fmt::Display for Stone {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Empty => ".",
            Self::Black => "B",
            Self::White => "W",
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Board {
    board: Vec<u8>,
}

impl Board {
    pub fn new() -> Self {
        const EMPTY_FILL: u8 = Stone::Empty as u8 * 0b01010101 ;
        Self {
            board: vec![EMPTY_FILL; 91],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Stone {
        if row >= 19 {
            panic!("`row` must be less than 19, but is {row}");
        }
        if col >= 19 {
            panic!("`col` must be less than 19, but is {col}");
        }

        // SAFETY: Both `row` and `col` are less than 19.
        unsafe { self.get_unchecked(row, col) }
    }

    /// # Safety
    /// Both `row` and `col` must be less than 19.
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> Stone {
        let bit_index = 2 * (19 * row + col);
        // SAFETY: If both `row` and `col` are less than 19, `bit_index / 8`
        // will be a valid index.
        let stone_code =
            (unsafe { self.board.get_unchecked(bit_index / 8) } >> (bit_index % 8)) & 0b11;
        // SAFETY: `Stone` is represented as a `u8`, so transmutation is safe as
        // long as the `u8` here is a `Stone` discriminant. The `u8` here is a
        // `Stone` discriminant, as we don't allow invalid bit patterns into
        // `Board.board`.
        unsafe { core::mem::transmute(stone_code) }
    }

    pub fn set(&mut self, row: usize, col: usize, stone: Stone) {
        if row >= 19 {
            panic!("`row` must be less than 19, but is {row}");
        }
        if col >= 19 {
            panic!("`col` must be less than 19, but is {col}");
        }

        // SAFETY: Both `row` and `col` are less than 19.
        unsafe { self.set_unchecked(row, col, stone) }
    }

    /// # Safety
    /// Both `row` and `col` must be less than 19.
    pub unsafe fn set_unchecked(&mut self, row: usize, col: usize, stone: Stone) {
        let bit_index = 2 * (19 * row + col);
        // SAFETY: If both `row` and `col` are less than 19, `bit_index / 8`
        // will be a valid index.
        let entry = unsafe { self.board.get_unchecked_mut(bit_index / 8) };
        let shift = bit_index % 8;
        *entry &= !(0b11 << shift);
        *entry |= (stone as u8) << shift;
    }

    pub fn get_board_mem(&self) {
        let data = &self.board;
        println!(
            "Memory size of data array: {} bytes",
            mem::size_of_val(data)
        );
        println!(
            "Total memory allocated by data: {} bytes",
            data.capacity() * mem::size_of::<u8>()
        );
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for row in 0..19 {
            for col in 0..19 {
                self.get(row, col).fmt(f)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
