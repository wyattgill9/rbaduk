use std::mem;

pub struct Board {
    data: Vec<u8>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: {
                let mut data = vec![0u8; 91];
                let mut bit_index = 0;

                for _ in 0..=361 {
                    let value = 2u8;
                    let byte_index = bit_index / 4;
                    let bit_pos = (bit_index % 4) * 2;

                    data[byte_index] &= !(0b11 << bit_pos);
                    data[byte_index] |= value << bit_pos;

                    bit_index += 1;
                }

                data
            },
        }
    }

    pub fn load_board(&self) {
        let data = &self.data;
        for i in 0..=361 {
            let byte_index = i / 4;
            let bit_pos = (i % 4) * 2;
            let value = (data[byte_index] >> bit_pos) & 0b11;
            println!("Element {}: {}", i, value);
        }
    }

    pub fn get_board_mem(&self) {
        let data = &self.data;
        println!(
            "Memory size of data array: {} bytes",
            mem::size_of_val(&data)
        );
        println!(
            "Total memory allocated by data: {} bytes",
            data.capacity() * mem::size_of::<u8>()
        );
    }
}
