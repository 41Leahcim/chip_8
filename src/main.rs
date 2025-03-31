use std::fs;

use chip_8::{instruction::Instruction, memory::Memory, registers::Registers};
use minifb::{Window, WindowOptions};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

fn main() {
    let mut window = Window::new(
        "Chip-8",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    let mut buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut memory = Memory::new();
    let mut registers = Registers::new();
    let application = fs::read("roms/RPS.ch8").unwrap();
    memory
        .slice_mut(0x200..0x1000)
        .unwrap()
        .copy_from_slice(&application);
    let mut pointer = 0x200;
    loop {
        if pointer >= 0xFFF {
            break;
        }
        let Ok(instruction) = Instruction::try_from(u16::from_le_bytes(
            memory
                .slice(pointer..pointer + 2)
                .unwrap()
                .try_into()
                .unwrap(),
        )) else {
            pointer += 2;
            continue;
        };
        match instruction {
            Instruction::SystemAddress(_) => continue,
            Instruction::ClearScreen => buffer.fill(0),
            Instruction::Return => pointer = memory.pop().unwrap() - 2,
            Instruction::JumpAddress(address) => pointer = address,
            Instruction::CallAddress(address) => {
                memory.push(pointer);
                pointer = address;
            }
            Instruction::SkipEqualRegByte(reg, byte) => {
                if registers.get_value(reg).unwrap() == byte {
                    pointer += 2;
                }
            }
            Instruction::SkipNotEqualRegByte(reg, byte) => {
                if registers.get_value(reg).unwrap() != byte {
                    pointer += 2;
                }
            }
            Instruction::SkipEqualRegisters(regs) => {
                if registers.get_value(regs & 0xF).unwrap()
                    == registers.get_value(regs >> 4).unwrap()
                {
                    pointer += 2;
                }
            }
            Instruction::LoadByte(reg, byte) => *registers.get_value_mut(reg).unwrap() = byte,
            Instruction::AddByte(reg, byte) => *registers.get_value_mut(reg).unwrap() += byte,
            Instruction::LoadRegister(reg) => {
                *registers.get_value_mut(reg >> 4).unwrap() =
                    registers.get_value(reg & 0xF).unwrap()
            }
            Instruction::Or(reg) => {
                *registers.get_value_mut(reg >> 4).unwrap() |=
                    registers.get_value(reg & 0xF).unwrap()
            }
            Instruction::And(reg) => {
                *registers.get_value_mut(reg >> 4).unwrap() &=
                    registers.get_value(reg & 0xF).unwrap()
            }
            Instruction::Xor(reg) => {
                *registers.get_value_mut(reg >> 4).unwrap() ^=
                    registers.get_value(reg & 0xF).unwrap()
            }
            Instruction::Add(_) => todo!(),
            Instruction::Sub(_) => todo!(),
            Instruction::ShiftRight(_) => todo!(),
            Instruction::SubInverted(_) => todo!(),
            Instruction::ShiftLeft(_) => todo!(),
            Instruction::SkipNotEqualReg(_) => todo!(),
            Instruction::LoadI(_) => todo!(),
            Instruction::JumpAddressOffset(_) => todo!(),
            Instruction::RandRange(_, _) => todo!(),
            Instruction::Draw(_, _) => todo!(),
            Instruction::SkipPressed(_) => todo!(),
            Instruction::SkipNotPressed(_) => todo!(),
            Instruction::LoadRegisterDelayTimer(_) => todo!(),
            Instruction::LoadKeyPress(_) => todo!(),
            Instruction::LoadDelayTimerRegister(_) => todo!(),
            Instruction::LoadSoundTimerRegister(_) => todo!(),
            Instruction::AddAddresssRegister(_) => todo!(),
            Instruction::LoadSpriteAddress(_) => todo!(),
            Instruction::LoadRegisterSprites(_) => todo!(),
            Instruction::LoadMemoryRegisters(_) => todo!(),
            Instruction::LoadRegistersMemory(_) => todo!(),
            Instruction::Exit => todo!(),
        }
        pointer += 2;
        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
