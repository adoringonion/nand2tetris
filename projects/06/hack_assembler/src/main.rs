use std::{env, fs::File, io::Write};

use anyhow::{Context, Result, anyhow};
use parser::{CommandType, Parser};

use crate::symbol_table::SymbolTable;
mod code;
mod parser;
mod symbol_table;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if !file_path.ends_with(".asm") {
        return Err(anyhow!("Invalid file extension: {}", file_path));
    }
    let file = File::open(file_path).with_context(|| format!("not find {}", file_path))?;

    let mut pre_parser = Parser::new(file)?;
    let mut parser = pre_parser.clone();
    let mut symbol_table = SymbolTable::new();

    // first pass
    let mut rom_address_counter = 0;
    loop {
        match pre_parser.command_type() {
            CommandType::LCommand => {
                let symbol = pre_parser.symbol();
                symbol_table.add_entry(&symbol, rom_address_counter);
            }
            _ => {
                rom_address_counter += 1;
            }
        }

        if !pre_parser.has_more_commands() {
            break;
        }

        pre_parser.advance();
    }

    // second pass
    let mut binary_vec: Vec<u16> = vec![];
    let mut ram_address_counter = 16;
    loop {
        match parser.command_type() {
            CommandType::ACommand => {
                let symbol_numeric = parser.symbol();
                if let Ok(num) = symbol_numeric.parse::<u16>() {
                    binary_vec.push(num);
                } else if symbol_table.contains(&symbol_numeric) {
                    binary_vec.push(symbol_table.get_address(&symbol_numeric));
                } else {
                    symbol_table.add_entry(&symbol_numeric, ram_address_counter);
                    binary_vec.push(ram_address_counter);
                    ram_address_counter += 1;
                }
            }
            CommandType::LCommand => { /*  nothing to do */ }
            CommandType::CCommand => {
                let dest = code::dest(&parser.dest());
                let comp = code::comp(&parser.comp());
                let jump = code::jump(&parser.jump());
                binary_vec.push(sum(dest, comp, jump));
            }
        }

        if !parser.has_more_commands() {
            break;
        }

        parser.advance();
    }

    write_file(&file_path, binary_vec)?;
    Ok(())
}

fn sum(dest: u16, comp: u16, jump: u16) -> u16 {
    0b1110000000000000 + (comp << 6) + (dest << 3) + jump
}

fn write_file(file_name: &str, vec: Vec<u16>) -> Result<()> {
    let output_file_name = file_name.replace(".asm", ".hack");
    let mut file = File::create(output_file_name)?;

    for binary in vec.iter() {
        println!("{:016b}", binary);
        file.write_fmt(format_args!("{:016b}\n", binary))?;
    }

    Ok(())
}
