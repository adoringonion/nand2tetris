use anyhow::{anyhow, Context, Result};
use std::{env, fs::File, path::Path};

use crate::parser::{CommandType, Parser};

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
    let mut code_writer = code_writer::CodeWriter::new(Path::new(file_path).file_stem().unwrap().to_str().unwrap());

    loop {
        match parser.command_type() {
            CommandType::CArithmetic => code_writer.write_arithmetic(&parser.arg1())?,
            CommandType::CPush | CommandType::CPop => {
                code_writer.write_push_pop(parser.command_type(), &parser.arg1(), parser.arg2())?
            },
            CommandType::CLabel => code_writer.write_label(&parser.arg1())?,
            CommandType::CGoto => code_writer.write_goto(&parser.arg1())?,
            CommandType::CIf => code_writer.write_if(&parser.arg1())?,
            CommandType::CCall => code_writer.write_call(&parser.arg1(), parser.arg2())?,
            CommandType::CReturn => code_writer.write_return()?,
            CommandType::CFunction => code_writer.write_function(&parser.arg1(), parser.arg2())?,
        }

        if !parser.has_more_commands() {
            break;
        }

        parser.advance();
    }

    Ok(())
}
