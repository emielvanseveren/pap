use crate::memory_operand::MemoryOperand;
use crate::register::Register;

pub enum Operand {
    // A register operand
    Register(Register),

    // A memory operand
    Memory(MemoryOperand),

    // An immediate operand
    Immediate(i16),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Operand::Register(reg) => write!(f, "{reg}"),
            // hex pretty print with "0x" prefix
            Operand::Immediate(imm) => write!(f, "{imm:#x}"),

            Operand::Memory(mem) => {
                // If there is a direct memory address, write it and return
                if let Some(address) = mem.address {
                    if let Some(size) = mem.size {
                        write!(f, "{size} ")?;
                    }

                    //if let Some(segment) = mem.segment {
                    //   write!(f, "{segment}:")?;
                    //}

                    write!(f, "[{address:#x}]")?;
                    return Ok(());
                }

                // Start the memory bracket
                if let Some(size) = mem.size {
                    write!(f, "{size} ")?;
                }

                // If a segment exists, insert it here
                //if let Some(segment) = mem.segment {
                //   write!(f, "{segment}:")?;
                //}

                // Open the memory bracket
                write!(f, "[")?;

                // If the operand has a register(s), write them
                if let Some(reg1) = mem.registers[0] {
                    write!(f, "{reg1}")?;

                    if let Some(reg2) = mem.registers[1] {
                        write!(f, " + {reg2}")?;
                    }
                }

                // Write the displacement if it exists
                if let Some(displacement) = mem.displacement {
                    if displacement != 0 {
                        // Add pretty spacing around the sign of the offset
                        if displacement.is_negative() {
                            write!(f, " - ")?;
                        } else {
                            write!(f, " + ")?;
                        }

                        // Regardless, print the absolute value of the offset
                        write!(f, "{:#x}", displacement.abs())?;
                    }
                }

                write!(f, "]")
            }
        }
    }
}

/// REG field parsed from an instruction stream
#[derive(Debug, Copy, Clone)]
pub struct Reg(pub u8);

// W field parsed from an instruciton stream
#[derive(Debug, Copy, Clone)]
pub struct Wide(pub u8);

/// RM field parsed from an instruction stream
#[derive(Debug, Copy, Clone)]
pub struct Rm(pub u8);

/// MOD field parsed from an instruction stream
// TODO: implement ref trait
#[derive(Debug, Copy, Clone)]
pub struct Mod(pub u8);

pub enum Instruction {
    Mov { src: Operand, dest: Operand },
}

impl std::fmt::Display for Instruction {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Instruction::Mov { dest, src } => {
                write!(f, "mov {dest}, {src}")
            }
        }
    }
}
