use anyhow::{anyhow, Result};
use std::{fs::File, io::Write};

use crate::parser::CommandType;

pub struct CodeWriter {
    pub file: File,
}

impl CodeWriter {
    pub fn new() -> Self {
        let file = File::create("out.asm").unwrap();
        CodeWriter { file }
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        let file = File::create(file_name);
        self.file = file.unwrap();
    }

    pub fn write_arithmetic(&mut self, command: &str) -> Result<()> {
        match command {
            "add" => Ok(self.file.write_fmt(format_args!(
                "
                @SP
                A=M-1
                D=M
                A=A-1
                M=M+D
                @SP
                M=M+1"
            ))?),
            "sub" => Ok(self.file.write_fmt(format_args!(
                "
                @SP
                A=M-1
                D=M
                A=A-1
                M=M-D
                @SP
                M=M+1"
            ))?),
            "neg" => Ok(self.file.write_fmt(format_args!(
                "   
                @SP
                A=M-1
                M=-M
                @SP
                M=M+1"
            ))?),
            "eq" => Ok(self.file.write_fmt(format_args!(
                "
                
            "))?),
            "gt" => Ok(self.file.write_fmt(format_args!(""))?),
            "lt" => Ok(self.file.write_fmt(format_args!(""))?),
            "and" => Ok(self.file.write_fmt(format_args!(
                "
                @SP
                A=M-1
                D=M
                A=A-1
                M=M&D
                @SP
                M=M+1"
            ))?),
            "or" => Ok(self.file.write_fmt(format_args!(
                "
                @SP
                A=M-1
                D=M
                A=A-1
                M=M|D
                @SP
                M=M+1"
            ))?),
            "not" => Ok(self.file.write_fmt(format_args!(
                "
                @SP
                A=M-1
                M=!M
                @SP
                M=M+1"
            ))?),
            _ => Err(anyhow!("Invalid arithmetic command: {}", command)),
        }
    }

    pub fn write_push_pop(
        &mut self,
        command: CommandType,
        segment: &str,
        index: u32,
    ) -> Result<()> {
        match command {
            CommandType::CPush => match segment {
                "constant" => Ok(self.file.write_fmt(format_args!(
                    "
                    @{}
                    D=A
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1",
                    index
                ))?),
                "local" => Ok(self.file.write_fmt(format_args!(
                    "
                    @{}
                    D=A
                    @LCL
                    A=M+D
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1
                    ",
                    index
                ))?),
                "argument" => Ok(self.file.write_fmt(format_args!(
                    "
                    @{}
                    D=A
                    @ARG
                    A=M+D
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1
                    ",
                    index
                ))?),
                "this" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @THIS
                    A=M+D
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1
                    ",
                    index
                ))?),
                "that" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @THAT
                    A=M+D
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1
                    ",
                    index
                ))?),
                "temp" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @5
                    A=A+D
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1
                    ",
                    index
                ))?),
                "pointer" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @3
                    A=A+D
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1
                    ",
                    index
                ))?),
                "static" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @16
                    A=A+D
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1
                    ",
                    index
                ))?),
                _ => Err(anyhow!("Invalid segment: {}", segment)),
            },
            CommandType::CPop => match segment {
                "local" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @LCL
                    D=M+D
                    @R13
                    M=D
                    @SP
                    AM=M-1
                    D=M
                    @R13
                    A=M
                    M=D
                    ",
                    index
                ))?),
                "argument" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @ARG
                    D=M+D
                    @R13
                    M=D
                    @SP
                    AM=M-1
                    D=M
                    @R13
                    A=M
                    M=D
                    ",
                    index
                ))?),
                "this" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @THIS
                    D=M+D
                    @R13
                    M=D
                    @SP
                    AM=M-1
                    D=M
                    @R13
                    A=M
                    M=D
                    ",
                    index
                ))?),
                "that" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @THAT
                    D=M+D
                    @R13
                    M=D
                    @SP
                    AM=M-1
                    D=M
                    @R13
                    A=M
                    M=D
                    ",
                    index
                ))?),
                "temp" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @5
                    D=A+D
                    @R13
                    M=D
                    @SP
                    AM=M-1
                    D=M
                    @R13
                    A=M
                    M=D
                    ",
                    index
                ))?),
                "pointer" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @3
                    D=A+D
                    @R13
                    M=D
                    @SP
                    AM=M-1
                    D=M
                    @R13
                    A=M
                    M=D
                    ",
                    index
                ))?),
                "static" => Ok(self.file.write_fmt(format_args!(
                    "@{}
                    D=A
                    @16
                    D=A+D
                    @R13
                    M=D
                    @SP
                    AM=M-1
                    D=M
                    @R13
                    A=M
                    M=D
                    ",
                    index
                ))?),
                _ => Err(anyhow!("Invalid segment: {}", segment)),
            },
            _ => Err(anyhow!("Invalid command: {:?}", command)),
        }
    }
}
