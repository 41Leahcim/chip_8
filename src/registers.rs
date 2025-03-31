//! This module contains the implementation for most registers.

use std::ops::Range;

/// The set of registers for the chip-8 architecture
pub struct Registers {
    /// The basic data registers (specified in the instructions)
    data: [u8; 16],

    /// The address or I register
    address: u16,

    /// The delay timer register
    delay: u8,

    /// The sound timer register
    sound: u8,
}

impl Registers {
    /// Retrieves the value of a general purpose register
    pub const fn get_value(&self, id: u8) -> Option<u8> {
        if id < 16 {
            Some(self.data[id as usize])
        } else {
            None
        }
    }

    /// Gets a mutable reference to a general purpose register
    pub const fn get_value_mut(&mut self, id: u8) -> Option<&mut u8> {
        if id < 16 {
            Some(&mut self.data[id as usize])
        } else {
            None
        }
    }

    /// Retrieves the value of the address register
    pub const fn address(&self) -> u16 {
        self.address
    }

    /// Gets a mutable reference to the address register
    pub const fn address_mut(&mut self) -> &mut u16 {
        &mut self.address
    }

    /// Retrieves the value of the delay timer
    pub const fn delay(&self) -> u8 {
        self.delay
    }

    /// Sets the delay timer to a new value
    pub const fn set_delay(&mut self, value: u8) {
        self.delay = value;
    }

    /// Retrievs the value of the sound timer
    pub const fn sound_timer(&self) -> u8 {
        self.sound
    }

    /// Sets the value of the sound timer
    pub const fn set_sound_timer(&mut self, value: u8) {
        self.sound = value;
    }

    /// Updates the timers
    pub const fn cycle(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    /// Takes a slice of the general purpose registers to load multiple bytes easily and quickly.
    pub fn slice(&self, range: Range<u16>) -> Option<&[u8]> {
        if range.end <= 0xFFF {
            Some(&self.data[range.start as usize..range.end as usize])
        } else {
            None
        }
    }

    /// Takes a mutable slice of the general purpose registers to store multiple bytes easily and
    /// quickly.
    pub fn slice_mut(&mut self, range: Range<u16>) -> Option<&mut [u8]> {
        if range.start >= 200 && range.end <= 0xFFF {
            Some(&mut self.data[range.start as usize..range.end as usize])
        } else {
            None
        }
    }
}
