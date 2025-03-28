use std::ops::{Index, IndexMut};

pub struct Memory {
    data: [u8; 0x1000],
    stack_pointer: u16,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub const fn new() -> Self {
        let mut data = [0; 0x1000];
        let sprites = [
            // 0
            [
                0b1111_0000,
                0b1001_0000,
                0b1001_0000,
                0b1001_0000,
                0b1111_0000,
            ],
            // 1
            [
                0b0010_0000,
                0b0110_0000,
                0b0010_0000,
                0b0010_0000,
                0b0111_0000,
            ],
            // 2
            [
                0b1111_0000,
                0b0001_0000,
                0b1111_0000,
                0b1000_0000,
                0b1111_0000,
            ],
            // 3
            [
                0b1111_0000,
                0b0001_0000,
                0b1111_0000,
                0b0001_0000,
                0b1111_0000,
            ],
            // 4
            [
                0b1001_0000,
                0b1001_0000,
                0b1111_0000,
                0b0001_0000,
                0b0001_0000,
            ],
            // 5
            [
                0b1111_0000,
                0b1000_0000,
                0b1111_0000,
                0b0001_0000,
                0b1111_0000,
            ],
            // 6
            [
                0b1111_0000,
                0b1000_0000,
                0b1111_0000,
                0b1001_0000,
                0b1111_0000,
            ],
            // 7
            [
                0b1111_0000,
                0b0001_0000,
                0b0010_0000,
                0b0100_0000,
                0b0100_0000,
            ],
            // 8
            [
                0b1111_0000,
                0b1001_0000,
                0b1111_0000,
                0b1001_0000,
                0b1111_0000,
            ],
            // 9
            [
                0b1111_0000,
                0b1001_0000,
                0b1111_0000,
                0b0001_0000,
                0b1111_0000,
            ],
            // A
            [
                0b1111_0000,
                0b1001_0000,
                0b1111_0000,
                0b1001_0000,
                0b1001_0000,
            ],
            // B
            [
                0b1110_0000,
                0b1001_0000,
                0b1110_0000,
                0b1001_0000,
                0b1110_0000,
            ],
            // C
            [
                0b1111_0000,
                0b1000_0000,
                0b1000_0000,
                0b1000_0000,
                0b1111_0000,
            ],
            // D
            [
                0b1110_0000,
                0b1001_0000,
                0b1001_0000,
                0b1001_0000,
                0b1110_0000,
            ],
            // E
            [
                0b1111_0000,
                0b1000_0000,
                0b1111_0000,
                0b1000_0000,
                0b1111_0000,
            ],
            // F
            [
                0b1111_0000,
                0b1000_0000,
                0b1111_0000,
                0b1000_0000,
                0b1000_0000,
            ],
        ];
        let mut sprite_index = 0;
        while sprite_index < sprites.len() {
            let mut byte_index = 0;
            let sprite = &sprites[sprite_index];
            while byte_index < sprite.len() {
                data[sprite_index * sprite.len() + byte_index] = sprite[byte_index];
                byte_index += 1;
            }
            sprite_index += 1;
        }
        Self {
            data,
            stack_pointer: (sprites.len() * sprites[0].len()) as u16,
        }
    }

    pub const fn load(&self, index: u16) -> Option<u8> {
        let index = index as usize;
        if index < self.data.len() {
            Some(self.data[index])
        } else {
            None
        }
    }

    pub const fn store(&mut self, index: u16, value: u8) -> bool {
        match index {
            ..0x200 | 0x1000.. => false,
            index => {
                self.data[index as usize] = value;
                true
            }
        }
    }

    pub fn push(&mut self, address: u16) -> bool {
        if self.stack_pointer + 2 >= 0x200 || !(0x200..0x1000).contains(&address) {
            return false;
        }
        let stack_pointer = self.stack_pointer as usize;
        self.data[stack_pointer..stack_pointer + 2].copy_from_slice(&address.to_ne_bytes());
        self.stack_pointer += 2;
        true
    }

    pub fn pop(&mut self) -> Option<u16> {
        if self.stack_pointer < 82 {
            return None;
        }
        self.stack_pointer -= 2;
        let mut word = [0; 2];
        word.copy_from_slice(&self.data[self.stack_pointer as usize..][..2]);
        Some(u16::from_ne_bytes(word))
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        self.data
            .get(usize::from(
                index.checked_sub(0x200).expect("Unreadable address"),
            ))
            .expect("Unreachable address")
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.data
            .get_mut(usize::from(
                index.checked_sub(0x200).expect("Unreadable address"),
            ))
            .expect("Unreachable address")
    }
}
