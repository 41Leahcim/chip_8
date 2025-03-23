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
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
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
            _ => todo!(),
        }
    }
}
