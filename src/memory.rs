//! This module contains the implementation of chip-8 memory

use std::ops::{Index, IndexMut, Range};

/// The memory struct contains the full chip-8 memory and stack pointer
pub struct Memory {
    /// The data stored in memory
    data: [u8; 0x1000],

    /// The current address of the end of the call stack
    stack_pointer: u16,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Initializes the memory
    pub const fn new() -> Self {
        // Data vector
        let mut data = [0; 0x1000];

        // The default sprites for numbers
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

        // Load the sprites into memory
        let mut sprite_index = 0;
        while sprite_index < sprites.len() {
            // Copy the current slice into memory
            let mut byte_index = 0;
            let sprite = &sprites[sprite_index];
            while byte_index < sprite.len() {
                data[sprite_index * sprite.len() + byte_index] = sprite[byte_index];
                byte_index += 1;
            }

            // Continue to the next slice
            sprite_index += 1;
        }

        // Create the memory object
        Self {
            data,
            stack_pointer: (sprites.len() * sprites[0].len()) as u16,
        }
    }

    /// Loads a value from memory if possible
    pub const fn load(&self, index: u16) -> Option<u8> {
        // Convert the index to a usize, so it can be compared to memory size and used as index
        let index = index as usize;

        // Return the requested byte if possible, None otherwise
        if index < self.data.len() {
            Some(self.data[index])
        } else {
            None
        }
    }

    /// Stores the requested byte if possible and allowed, returns whether the value was stored.
    pub const fn store(&mut self, index: u16, value: u8) -> bool {
        match index {
            // If the index points to protected memory or non-existing, the value can't be stored.
            ..0x200 | 0x1000.. => false,

            // Otherwise, set it
            index => {
                self.data[index as usize] = value;
                true
            }
        }
    }

    /// Pushes a new code address on the stack
    pub fn push(&mut self, address: u16) -> bool {
        // Only data and code addresses can be stored.
        // Return false for other addresses or if the stack is full
        if self.stack_pointer + 2 >= 0x200 || !(0x200..0x1000).contains(&address) {
            return false;
        }

        // Convert the stack pointer to a usize, so it can actually be used as an index
        let stack_pointer = self.stack_pointer as usize;

        // Copy the address to the location pointed to by the stack pointer
        self.data[stack_pointer..stack_pointer + 2].copy_from_slice(&address.to_ne_bytes());

        // Increment the stack pointer
        self.stack_pointer += 2;
        true
    }

    /// Pops an address from the stack
    pub fn pop(&mut self) -> Option<u16> {
        // Return None if no address has been stored on the stack yet
        if self.stack_pointer < 82 {
            return None;
        }

        // Decrement the stack pointer
        self.stack_pointer -= 2;

        // Load the memory from the stack and convert it to an address again
        let mut word = [0; 2];
        word.copy_from_slice(&self.data[self.stack_pointer as usize..][..2]);
        Some(u16::from_ne_bytes(word))
    }

    /// Takes a slice of memory to load multiple bytes easily and quickly
    pub fn slice(&self, range: Range<u16>) -> Option<&[u8]> {
        if range.end <= 0xFFF {
            Some(&self.data[range.start as usize..range.end as usize])
        } else {
            None
        }
    }

    /// Takes a mutable slice of memory to store multiple bytes easily and quickly
    pub fn slice_mut(&mut self, range: Range<u16>) -> Option<&mut [u8]> {
        if range.start >= 200 && range.end <= 0xFFF {
            Some(&mut self.data[range.start as usize..range.end as usize])
        } else {
            None
        }
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        // Load the data from the st
        self.data
            .get(usize::from(index))
            .expect("Unreachable address")
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        assert!(
            (0x200..0x1000).contains(&index),
            "Invalid mutable reference to read-only or non-existing memory: {index}"
        );
        self.data
            .get_mut(usize::from(index))
            .expect("Unreachable address")
    }
}
