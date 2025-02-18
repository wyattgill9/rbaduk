use std::mem;

pub enum Stone {
    Empty,
    Black,
    White,
}

pub struct Board {
    data: Vec<u8>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: {
                let data = vec![0b00000000; 91];
                data
            },
        }
    }

    pub fn place_stone(&mut self, pos: (u8, u8), stone: Stone) {
        let index = (pos.0 * 19 + pos.1) as usize;
        let u8_index = index / 4;
        let bit_offset = (index % 4) * 2;

        let stone_value = match stone {
            Stone::Empty => 0b00,
            Stone::Black => 0b01,
            Stone::White => 0b10,
        };

        // mask to clear the existing 2 bits at the pos
        let mask = !(0b11 << bit_offset);

        //clear the existing bits and set new stone value
        self.data[u8_index] &= mask;
        self.data[u8_index] |= stone_value << bit_offset;
    }

    pub fn get_board_mem(&self) {
        let data = &self.data;
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

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..19 {
            for col in 0..19 {
                let index = (row * 19 + col) as usize;
                let u8_index = index / 4;
                let bit_offset = (index % 4) * 2;

                let stone_value = (self.data[u8_index] >> bit_offset) & 0b11;

                let display_char = match stone_value {
                    0b01 => 'B',
                    0b10 => 'W',
                    _ => '.',
                };

                write!(f, "{} ", display_char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
