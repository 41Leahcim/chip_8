//! This module contains the implementation of the instruction set

/// This enum contains all supported instructions
pub enum Instruction {
    /// Jump to a machine code routine at the specified address (12-bit).
    /// This instruction is only used on the old computers on which Chip-8 was originally
    /// implemented.
    /// It's ignored by modern interpreters.
    SystemAddress(u16),

    /// Clear the display
    ClearScreen,

    /// Return from a subroutine.
    /// The interpreter set the program counter to the address at the top of the stack, then
    /// subtracts 1 from the stack pointer.
    Return,

    /// Jump to the specified location (12-bit).
    /// The interpreter sets the program counter to the specified location.
    JumpAddress(u16),

    /// Call subroutine at the specified location (12-bit).
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the
    /// stack.
    /// The PC is then sets to the specified address.
    CallAddress(u16),

    /// Skip the next instruction if the register is equal to the byte.
    /// First value is the register id (16 registers), second is the byte to compare to.
    SkipEqualRegByte(u8, u8),

    /// Skip the next instruction if the register is not equal to the byte.
    /// First value is the register id (16 registers), second is the byte to compare to.
    SkipNotEqualRegByte(u8, u8),

    /// Skip the next instruction if the registers are equal.
    /// The 4 most significant bits indicate the register, the 4 least significant bits are ignored
    SkipEqualRegisters(u8),

    /// Loads the byte into the register.
    /// First byte indicates the register (4-bit), second contains the byte to load.
    LoadByte(u8, u8),

    /// Adds the byte (second byte), to the value of the register (first byte, 4 bits).
    AddByte(u8, u8),

    /// Stores the value of register vy in register Vx.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    LoadRegister(u8),

    /// Performs a bitwise or on the values of Vx and Vy, then stores the result in Vx.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    Or(u8),

    /// Performs a bitwise and on the values of Vx and Vy, then stores the result in Vx.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    And(u8),

    /// Performs a bitwise xor on the values of Vx and Vy, then stores the result in Vx.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    Xor(u8),

    /// Adds the values of Vx and Vy, then stores the result in Vx.
    /// If the result is greater than 255 (8-bits), VF is set to 1, otherwise to 0.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    Add(u8),

    /// If vx > vy, vf is set to 1, otherwise 0.
    /// Then vy is subtracted from vx and the result is stored in vx.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    Sub(u8),

    /// If the least significant bit of vx is 1, set VF to 1, otherwise 0.
    /// Then vx is divided by 2.
    /// The 4 most significant bits indicate the register, the 4 least significant bits are ignored
    ShiftRight(u8),

    /// If vy > vx, then VF is set to 1, otherwise 0.
    /// Then vx is subtracted from vy, and the result is stored in vx.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    SubInverted(u8),

    /// If the most significant bit of vx is set to 1, then VF is set to 1, otherwise 0.
    /// Then vx is multiplied by 2.
    /// The 4 most significant bits indicate the register, the 4 least significant bits are ignored
    ShiftLeft(u8),

    /// Skip next instruction if the values of the registers are not equal.
    /// The 4 most significant bits indicate register x, least significant represent register y.
    SkipNotEqualReg(u8),

    /// Load the 12-bit address into register I.
    LoadI(u16),

    /// Add the value of register 0 to the address and jump to the resulting address.
    JumpAddressOffset(u16),

    /// Generates a random number, ands it with the second byte and stores the result in the
    /// register indicated by the least significant 4-bits of the first byte.
    RandRange(u8, u8),

    /// Most significant 4-bits of first byte indicate the x and most significant 4-bits of the
    /// first byte indicate the y position of the location to start drawing at.
    /// Draws a number of bytes specified by the least significant 4-bits of the second byte.
    /// Reads the data to draw from the address indicated by the value of the I register.
    /// The data is read and displayed as a sprite.
    /// All data of a sprite is stored consecutively, but displayed as a 5 high x 8 wide image.
    /// The sprite is drawn by xoring it with the data already stored in that position.
    /// If any pixels were erased, VF is set to 1, otherwise it's set to 0.
    /// If a part of the sprite is outside the display coordinates, it wraps around to the oposite
    /// side of the screen.
    Draw(u8, u8),

    /// Skips next instruction if the key with the value of the least significant 4 bits is
    /// pressed. Valid keys are 0 - 9 and A - F (case insensitive, ranges inclusive).
    SkipPressed(u8),

    /// Skips next instruction if the key with the value of the least significant 4 bits isn't
    /// pressed. Valid keys are 0 - 9 and A - F (case insensitive, ranges inclusive).
    SkipNotPressed(u8),

    /// Loads the delay timer value into the register (4-bit).
    LoadRegisterDelayTimer(u8),

    /// Waits for a key to be pressed and stores the value of the pressed key into the register.
    LoadKeyPress(u8),

    /// Loads the value of the register into the delay timer.
    LoadDelayTimerRegister(u8),

    /// Loads the value of the register into the sound timer.
    LoadSoundTimerRegister(u8),

    /// Add the value of the register to the address register.
    AddAddresssRegister(u8),

    /// Loads the address of the sprite for the value of the register
    LoadSpriteAddress(u8),
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        // Decode the instruction word
        match value {
            0x00E0 => Self::ClearScreen,
            0x00EE => Self::Return,
            0..=0xFFF => Self::SystemAddress(value),
            0x1000..=0x1FFF => Self::JumpAddress(value & 0xFFF),
            0x2000..=0x2FFF => Self::CallAddress(value & 0xFFF),
            0x3000..=0x3FFF => Self::SkipEqualRegByte((value >> 8) as u8 & 0xF, value as u8),
            0x4000..=0x4FFF => Self::SkipNotEqualRegByte((value >> 8) as u8 & 0xF, value as u8),
            0x5000..=0x5FFF if value & 0xF == 0 => Self::SkipEqualRegisters((value >> 4) as u8),
            0x6000..=0x6FFF => Self::LoadByte((value >> 8) as u8, value as u8),
            0x7000..=0x7FFF => Self::AddByte((value >> 8) as u8 & 0xF, value as u8),
            0x8000..=0x8FFF if value & 0xF == 0 => Self::LoadRegister((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 1 => Self::Or((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 2 => Self::And((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 3 => Self::Xor((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 4 => Self::Add((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 5 => Self::Sub((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 6 => Self::ShiftRight((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 7 => Self::SubInverted((value >> 4) as u8),
            0x8000..=0x8FFF if value & 0xF == 0xE => Self::ShiftLeft((value >> 4) as u8),
            0x9000..=0x9FFF if value & 0xF == 0 => Self::SkipNotEqualReg((value >> 4) as u8),
            0xA000..=0xAFFF => Self::LoadI(value & 0xFFF),
            0xB000..=0xBFFF => Self::JumpAddressOffset(value & 0xFFF),
            0xC000..=0xCFFF => Self::RandRange((value >> 8) as u8 & 0xF, value as u8),
            0xD000..=0xDFFF => Self::Draw((value >> 8) as u8, value as u8),
            0xE000..=0xEFFF if value & 0xFF == 0x9E => Self::SkipPressed((value >> 8) as u8 & 0xF),
            0xE000..=0xEFFF if value & 0xFF == 0xA1 => {
                Self::SkipNotPressed((value >> 8) as u8 & 0xF)
            }
            0xF000..=0xFFFF if value & 0xFF == 0x07 => {
                Self::LoadRegisterDelayTimer((value >> 8) as u8 & 0xF)
            }
            0xF000..=0xFFFF if value & 0xFF == 0x0A => Self::LoadKeyPress((value >> 8) as u8 & 0xF),
            0xF000..=0xFFFF if value & 0xFF == 0x15 => {
                Self::LoadDelayTimerRegister((value >> 8) as u8 & 0xF)
            }
            0xF000..=0xFFFF if value & 0xFF == 0x18 => {
                Self::LoadSoundTimerRegister((value >> 8) as u8 & 0xF)
            }
            0xF000..=0xFFFF if value & 0xFF == 0x1E => {
                Self::AddAddresssRegister((value >> 8) as u8 & 0xF)
            }
            0xF000..=0xFFFF if value & 0xFF == 0x29 => {
                Self::LoadSpriteAddress((value >> 8) as u8 & 0xF)
            }
            _ => todo!(),
        }
    }
}
