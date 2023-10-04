use crate::instruction::{Mod, Rm, Wide};
use crate::register::Register;
use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemorySize {
    Byte,
    Word,
}

impl std::fmt::Display for MemorySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MemorySize::Byte => write!(f, "byte"),
            MemorySize::Word => write!(f, "word"),
        }
    }
}

/// A memory operand
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryOperand {
    /// Registers involved with this memory operand
    pub registers: [Option<Register>; 2],

    /// Displacement value for this memory operand
    pub displacement: Option<i16>,

    /// Size of memory read
    pub size: Option<MemorySize>,

    /// Direct address for this memory operand
    pub address: Option<u16>,
}

impl MemoryOperand {
    /// Create a direct address memory operand
    pub fn direct_address(addr: u16, wide: Wide) -> Self {
        let size = match wide {
            Wide(0) => MemorySize::Byte,
            Wide(1) => MemorySize::Word,
            _ => unsafe { std::hint::unreachable_unchecked() },
        };

        Self {
            registers: [None; 2],
            displacement: None,
            size: Some(size),
            address: Some(addr),
        }
    }

    pub fn from_mod_rm(mod_: Mod, rm: Rm, w: Wide) -> Result<Self> {
        let mut registers = [None; 2];

        // (Register/Memory) Field decoding
        // Based on table 4-20 intel manual
        match rm.0 {
            0b000 => {
                registers[0] = Some(Register::Bx);
                registers[1] = Some(Register::Si);
            }
            0b001 => {
                registers[0] = Some(Register::Bx);
                registers[1] = Some(Register::Di);
            }
            0b010 => {
                registers[0] = Some(Register::Bp);
                registers[1] = Some(Register::Si);
            }
            0b011 => {
                registers[0] = Some(Register::Bp);
                registers[1] = Some(Register::Di);
            }
            0b100 => {
                registers[0] = Some(Register::Si);
            }
            0b101 => {
                registers[0] = Some(Register::Di);
            }
            0b110 => {
                // 16 bit displacement

                todo!("direct address")
            }
            0b111 => {
                registers[0] = Some(Register::Bx);
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }

        let size = match w.0 {
            0 => MemorySize::Byte,
            1 => MemorySize::Word,
            _ => unsafe { std::hint::unreachable_unchecked() },
        };

        Ok(Self {
            registers,
            size: Some(size),
            address: None,
            displacement: None,
        })
    }

    pub fn with_displacement(&mut self, displacement: i16) -> Self {
        self.displacement = Some(displacement);
        *self
    }
}
