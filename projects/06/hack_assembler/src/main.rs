use std::{env, fs::File, io::Write};

use anyhow::Result;
use parser::{CommandType, Parser};
mod code;
mod parser;
mod symbol_table;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let mut parser = Parser::new(&input_file)?;
    let mut binary_vec: Vec<u16> = vec![];

    parser.advance();
    parser.advance();

    loop {

        match parser.command_type() {
            CommandType::ACommand | CommandType::LCommand => {
                let symbol_numeric = parser.symbol();
                if let Ok(num) = symbol_numeric.parse::<u16>() {
                    binary_vec.push(num);
                }
            }
            CommandType::CCommand => {
                let dest = code::dest(&parser.dest());
                let comp = code::comp(&parser.comp());
                let jump = code::jump(&parser.jump());
                binary_vec.push(conv(dest, comp, jump));
            }
        }

        if !parser.has_more_commands() {
            break;
        }

        parser.advance();
    }

    write_file(&input_file, binary_vec)?;
    Ok(())
}

fn conv(dest: u16, comp: u16, jump: u16) -> u16 {
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
