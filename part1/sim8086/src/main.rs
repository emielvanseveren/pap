use anyhow::Result;
use memory_operand::MemoryOperand;
use std::io::{self, BufReader, Read, Write};
use std::slice::Iter;

mod instruction;
mod memory_operand;
mod register;

use instruction::Instruction;

use crate::instruction::{Mod, Operand, Reg, Rm, Wide};
use crate::register::Register;

fn main() -> Result<()> {
    // the binary takes a filepath
    let path = std::env::args().nth(1).expect("no path given");

    let f = std::fs::File::open(path)?;
    let mut reader = BufReader::new(f);

    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;

    println!("{:?}", buffer);

    let asm = decode(&buffer)?;
    io::stdout().write_all(asm.as_bytes())?;

    Ok(())
}

fn decode(bytes: &[u8]) -> Result<String> {
    let mut asm = String::from("bits 16\n\n");
    let mut iter = bytes.iter();

    while let Some(b1) = iter.next() {
        println!("{:08b}", b1);

        // the first x amount of bits define the opcode and variant.
        let inst = match *b1 {
            // from-to
             0b1000_1000..=0b1000_1011   // mov

            => {
                // 0 = instruction operates on byte data
                // 1 = instruction operates on word data
                let w = *b1 & 0b0000_0001;
                let d = (*b1 & 0b0000_0010) >> 1 == 1;

                let b2 = iter.next().expect("expected second byte");
                let (mut reg1, mut reg2) = parse_mod_reg_rm_instr(&mut iter, *b2, Wide(w))?;

                // Direction field
                // 0 = Instruction source is specified in REG field
                // 1 = Instruction destination is specified in REG field
                if !d {
                    std::mem::swap(&mut reg1, &mut reg2);
                }
                Instruction::Mov { dest: reg1, src: reg2}
            },

            // Immediate to register
            0b1011_0000..=0b10111111 => {
                let reg = *b1 & 0b0000_0111;
                let w = (*b1 & 0b0000_1000) >> 3;
                let imm = if w == 1 {
                    i16::from_le_bytes([*iter.next().expect("immediate byte"), 0])
                } else {
                    i16::from_le_bytes([*iter.next().expect("lower immediate byte"), *iter.next().expect("higher immediate byte")])
                };
                Instruction::Mov { dest: Operand::Register(Register::from_reg_w(Reg(reg), Wide(w))), src: Operand::Immediate(imm) }
            }


            // Immediate to register/memory
            0b1100_0110 | 0b1100_0111 => {

                let b2 = *iter.next().expect("second byte");
                let w = *b1 & 1;

                let mod_ = Mod(b2 >> 6);
                let rm = Rm(b2 & 0b111);

                match mod_.0 {
                    0b00 => { todo!()},
                    0b01 => { todo!()},
                    0b10 => { todo!()},
                    0b11 => { todo!()},
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
                
            },

            // Mov memory to accumulator
            0b10100000 | 0b10100001 => {
                let w= *b1 & 1;
                let imm = if w == 1 {
                    u16::from_le_bytes([*iter.next().expect("address low"), 0])
                } else {
                    u16::from_le_bytes([*iter.next().expect("address low"), *iter.next().expect("address high")])
                };
                Instruction::Mov { dest: Operand::Register(Register::Ax), src: Operand::Memory(MemoryOperand::direct_address(imm, Wide(w)))}
            },

            // Mov accumulator to memory
            0b10100010 | 0b10100011 => {
                let w = *b1 & 1;
                let imm = if w == 1 {
                    u16::from_le_bytes([*iter.next().expect("address low"), 0])
                } else {
                    u16::from_le_bytes([*iter.next().expect("address low"), *iter.next().expect("address high")])
                };
                Instruction::Mov { dest: Operand::Memory(MemoryOperand::direct_address(imm, Wide(w))), src: Operand::Register(Register::Ax)}
            },

            // Register/memory to segment register
            0b10001110 =>{ todo!()},
            // Segment register to register/memory
            0b10001100 =>{ todo!()},
            _ => panic!("unimplemented opcode"),
        };
        asm.push_str(&format!("{}\n", inst));
    }

    Ok(asm)
}

/// Parse byte with "mod|reg|r/m" bit pattern
fn parse_mod_reg_rm_instr(iter: &mut Iter<u8>, b: u8, w: Wide) -> Result<(Operand, Operand)> {
    let rm = Rm(b & 0b111);
    let reg = Reg(b >> 3 & 0b111);
    // indicates whether one of the operands is in memory or whether both operands are registers
    // basically indicates how many displacmeent bytes are present
    let mod_ = Mod((b >> 6) & 0b11);

    Ok(match mod_.0 {
        // Memory mode, no displacement (except when R/M = 110)
        0b00 => {
            // exception: when R/M = 110, 16 bit displacement follows
            let mem: MemoryOperand = if rm.0 == 0b110 {
                let address = u16::from_le_bytes([
                    *iter.next().expect("lower address byte"),
                    *iter.next().expect("higher address byte"),
                ]);
                MemoryOperand::direct_address(address, w)
            } else {
                // No displacement
                MemoryOperand::from_mod_rm(mod_, rm, w)?
            };
            (
                Operand::Register(Register::from_reg_w(reg, w)),
                Operand::Memory(mem),
            )
        }
        // Memory mode, 8-bit displacement
        0b01 => {
            let reg = Register::from_reg_w(reg, w);
            let displacement = i16::from_le_bytes([*iter.next().expect("displacement byte"), 0]);
            let mem = MemoryOperand::from_mod_rm(mod_, rm, w)?.with_displacement(displacement);
            (Operand::Register(reg), Operand::Memory(mem))
        }
        // Memory mode, 16-bit displacement
        0b10 => {
            let reg = Register::from_reg_w(reg, w);
            let displacement = i16::from_le_bytes([
                *iter.next().expect("lower displacement byte"),
                *iter.next().expect("higher displacement byte"),
            ]);
            let mem = MemoryOperand::from_mod_rm(mod_, rm, w)?.with_displacement(displacement);
            (Operand::Register(reg), Operand::Memory(mem))
        }
        // Register Mode (no displacement)
        0b11 => {
            let reg = Register::from_reg_w(reg, w);
            let rm_reg = Register::from_reg_w(Reg(rm.0), w);
            (Operand::Register(reg), Operand::Register(rm_reg))
        }
        _ => unsafe { std::hint::unreachable_unchecked() },
    })
}
