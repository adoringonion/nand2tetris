use anyhow::{anyhow, Context, Result};
use std::{
    env,
    fs::{self, File},
    path::Path,
};

use crate::parser::{CommandType, Parser};

mod code_writer;
mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);

    if file_path.is_dir() {
        return Ok(multi_vm(&file_path)?);
    } else if file_path.extension().unwrap() == "vm" {
        return Ok(single_vm(file_path)?);
    } else {
        return Err(anyhow!("Invalid file path"));
    };
}

fn single_vm(file_path: &Path) -> Result<()> {
    let mut code_writer =
        code_writer::CodeWriter::new(file_path.file_stem().unwrap().to_str().unwrap());
    code_writer.write_init()?;
    let file = File::open(file_path)?;
    let mut parser = Parser::new(file);

    loop {
        match parser.command_type() {
            CommandType::CArithmetic => code_writer.write_arithmetic(&parser.arg1())?,
            CommandType::CPush | CommandType::CPop => {
                code_writer.write_push_pop(parser.command_type(), &parser.arg1(), parser.arg2())?
            }
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

    loop {
        match parser.command_type() {
            CommandType::CArithmetic => code_writer.write_arithmetic(&parser.arg1())?,
            CommandType::CPush | CommandType::CPop => {
                code_writer.write_push_pop(parser.command_type(), &parser.arg1(), parser.arg2())?
            }
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

fn multi_vm(file_path: &Path) -> Result<()> {
    let files = fs::read_dir(file_path).context("Failed to read directory")?;
    let mut code_writer =
        code_writer::CodeWriter::new(file_path.file_stem().unwrap().to_str().unwrap());
    code_writer.write_init()?;

    for vm in files.into_iter() {
        let vm = vm.context("Failed to read vm file")?;
        if vm.path().extension().unwrap() != "vm" {
            continue;
        }

        let file = File::open(vm.path())?;
        let mut parser = Parser::new(file);
        code_writer.set_file_name(vm.path().file_name().unwrap().to_str().unwrap());

        loop {
            match parser.command_type() {
                CommandType::CArithmetic => code_writer.write_arithmetic(&parser.arg1())?,
                CommandType::CPush | CommandType::CPop => code_writer.write_push_pop(
                    parser.command_type(),
                    &parser.arg1(),
                    parser.arg2(),
                )?,
                CommandType::CLabel => code_writer.write_label(&parser.arg1())?,
                CommandType::CGoto => code_writer.write_goto(&parser.arg1())?,
                CommandType::CIf => code_writer.write_if(&parser.arg1())?,
                CommandType::CCall => code_writer.write_call(&parser.arg1(), parser.arg2())?,
                CommandType::CReturn => code_writer.write_return()?,
                CommandType::CFunction => {
                    code_writer.write_function(&parser.arg1(), parser.arg2())?
                }
            }

            if !parser.has_more_commands() {
                break;
            }

            parser.advance();
        }
    }

    Ok(())
}
