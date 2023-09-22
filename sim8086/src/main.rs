use std::io::{self, BufReader, Read, Write};

fn main() -> io::Result<()> {
    // the binary takes a filepath
    let path = std::env::args().nth(1).expect("no path given");

    let f = std::fs::File::open(path)?;
    let mut reader = BufReader::new(f);

    let asm = decode(&mut reader)?;

    io::stdout().write_all(asm.as_bytes())?;

    Ok(())
}

fn decode(reader: &mut BufReader<std::fs::File>) -> io::Result<String> {
    let mut buffer = [0u8; 2];

    // we start by assuming the file is a 16-bit
    let mut asm = String::from("bits 16\n\n");

    // read the first two bytes
    while reader.read(&mut buffer)? > 0 {
        // second bit is direction field, we don't need it yet
        let d = buffer[0] & 0b0000_0010 != 0;
        // wide
        let w = buffer[0] & 0b0000_0001 != 0;

        let reg_1 = match (buffer[1], w) {
            (b, false) if b & 0b00111000 == 0b00000000 => "al".to_string(),
            (b, false) if b & 0b00111000 == 0b00001000 => "cl".to_string(),
            (b, false) if b & 0b00111000 == 0b00010000 => "dl".to_string(),
            (b, false) if b & 0b00111000 == 0b00011000 => "bl".to_string(),
            (b, false) if b & 0b00111000 == 0b00100000 => "ah".to_string(),
            (b, false) if b & 0b00111000 == 0b00101000 => "ch".to_string(),
            (b, false) if b & 0b00111000 == 0b00110000 => "dh".to_string(),
            (b, false) if b & 0b00111000 == 0b00111000 => "bh".to_string(),
            (b, true) if b & 0b00111000 == 0b00000000 => "ax".to_string(),
            (b, true) if b & 0b00111000 == 0b00001000 => "cx".to_string(),
            (b, true) if b & 0b00111000 == 0b00010000 => "dx".to_string(),
            (b, true) if b & 0b00111000 == 0b00011000 => "bx".to_string(),
            (b, true) if b & 0b00111000 == 0b00100000 => "sp".to_string(),
            (b, true) if b & 0b00111000 == 0b00101000 => "bp".to_string(),
            (b, true) if b & 0b00111000 == 0b00110000 => "si".to_string(),
            (b, true) if b & 0b00111000 == 0b00111000 => "di".to_string(),
            _ => panic!(
                "Unhandled (b, wide) combination: ({:08b}, {})",
                buffer[1], w
            ),
        };

        let reg_2 = match (buffer[1], w) {
            (b, false) if b & 0b00000111 == 0b00000000 => "al".to_string(),
            (b, false) if b & 0b00000111 == 0b00000001 => "cl".to_string(),
            (b, false) if b & 0b00000111 == 0b00000010 => "dl".to_string(),
            (b, false) if b & 0b00000111 == 0b00000011 => "bl".to_string(),
            (b, false) if b & 0b00000111 == 0b00000100 => "ah".to_string(),
            (b, false) if b & 0b00000111 == 0b00000101 => "ch".to_string(),
            (b, false) if b & 0b00000111 == 0b00000110 => "dh".to_string(),
            (b, false) if b & 0b00000111 == 0b00000111 => "bh".to_string(),
            (b, true) if b & 0b00000111 == 0b00000000 => "ax".to_string(),
            (b, true) if b & 0b00000111 == 0b00000001 => "cx".to_string(),
            (b, true) if b & 0b00000111 == 0b00000010 => "dx".to_string(),
            (b, true) if b & 0b00000111 == 0b00000011 => "bx".to_string(),
            (b, true) if b & 0b00000111 == 0b00000100 => "sp".to_string(),
            (b, true) if b & 0b00000111 == 0b00000101 => "bp".to_string(),
            (b, true) if b & 0b00000111 == 0b00000110 => "si".to_string(),
            (b, true) if b & 0b00000111 == 0b00000111 => "di".to_string(),
            _ => panic!(
                "Unhandled (b, wide) combination: ({:08b}, {})",
                buffer[1], w
            ),
        };

        // single instruction
        asm.push_str("mov ");

        if d {
            asm.push_str(&reg_1);
            asm.push_str(", ");
            asm.push_str(&reg_2);
        } else {
            asm.push_str(&reg_2);
            asm.push_str(", ");
            asm.push_str(&reg_1);
        }
        asm.push('\n');
    }

    Ok(asm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reg_reg_mov() {}
}
