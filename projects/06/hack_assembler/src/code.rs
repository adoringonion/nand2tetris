pub fn dest(mnemonic: &str) -> u8 {
    match mnemonic {
        "M" => 0b001,
        "D" => 0b010,
        "MD" => 0b011,
        "A" => 0b100,
        "AM" => 0b101,
        "AD" => 0b110,
        "AMD" => 0b111,
        _ => 0b000,
    }
}

pub fn jump(mnemonic: &str) -> i8 {
    match mnemonic {
        "JGT" => 0b000,
        "JEQ" => 0b010,
        "JGE" => 0b011,
        "JLT" => 0b100,
        "JNE" => 0b101,
        "JLE" => 0b110,
        "JMP" => 0b111,
        _ => 0b000,
    }
}

pub fn comp(mnemonic: &str) -> i8 {
    match mnemonic {
        "0" => 0b101010,
        "1" => 0b111111,
        "-1" => 0b111010,
        "D" => 0b001100,
        "A" | "M" => 0b110000,
        "!D" => 0b001101,
        "!A" | "!M" => 0b110001,
        "-D" => 0b001111,
        "-A" | "-M" => 0b110011,
        "D+1" => 0b011111,
        "A+1" | "M+1" => 0b110111,
        "D-1" => 0b001110,
        "A-1" | "M-1" => 0b110010,
        "D+A" | "D+M" => 0b000010,
        "D-A" | "D-M" => 0b010011,
        "A-D" | "M-D" => 0b000111,
        "D&A" | "D&M" => 0b000000,
        "D|A" | "D|M" => 0b010101,
        _ => 0,
    }
}
