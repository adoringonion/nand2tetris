use anyhow::{anyhow, Context, Result};
use std::{env, fs::File};

use crate::parser::Parser;

mod code_writer;
mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if !file_path.ends_with(".vm") {
        return Err(anyhow!("Invalid file extension: {}", file_path));
    }
    let file = File::open(file_path).with_context(|| format!("not find {}", file_path))?;

    let mut parser = Parser::new(file);

    loop {
        match parser.command_type() {
            parser::CommandType::CArithmetic => todo!(),
            parser::CommandType::CPush => todo!(),
            _ => todo!(),
        }

        if !parser.has_more_commands() {
            break;
        }

        parser.advance();
    }

    Ok(())
}
