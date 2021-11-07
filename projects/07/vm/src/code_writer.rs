use anyhow::{anyhow, Result};
use std::{fs::File, io::Write};

use crate::parser::CommandType;

pub struct CodeWriter {
    file_name: String,
    pub file: File,
    label_count: usize,
}

impl CodeWriter {
    pub fn new(file_name: &str) -> Self {
        let file = File::create(format!("{}.asm", file_name)).unwrap();
        CodeWriter {
            file_name: file_name.to_string(),
            file,
            label_count: 0,
        }
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        let file = File::create(file_name);
        self.file = file.unwrap();
    }

    pub fn write_arithmetic(&mut self, command: &str) -> Result<()> {
        match command {
            "add" => Ok(self.file.write_fmt(format_args!(
                "@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1
"
            ))?),
            "sub" => Ok(self.file.write_fmt(format_args!(
                "@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1
"
            ))?),
            "neg" => Ok(self.file.write_fmt(format_args!(
                "@SP
M=M-1
A=M
M=-M
@SP
M=M+1
"
            ))?),
            "eq" => {
                self.label_count += 1;
                Ok(self.file.write_fmt(format_args!(
                    "@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@EQ.{}
D;JEQ
D=0
@SP
A=M
M=D
@SP
M=M+1
@CONTINUE.{}
0;JMP
(EQ.{})
@0
D=A
D=D-1
@SP
A=M
M=D
@SP
M=M+1
(CONTINUE.{})
",
                    self.label_count, self.label_count, self.label_count, self.label_count
                ))?)
            }
            "gt" => {
                self.label_count += 1;
                Ok(self.file.write_fmt(format_args!(
                    "@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@GT.{}
D;JGT
D=0
@SP
A=M
M=D
@SP
M=M+1
@CONTINUE.{}
0;JMP
(GT.{})
D=0
D=D-1
@SP
A=M
M=D
@SP
M=M+1
(CONTINUE.{})
",
                    self.label_count, self.label_count, self.label_count, self.label_count
                ))?)
            }
            "lt" => {
                self.label_count += 1;
                Ok(self.file.write_fmt(format_args!(
                    "@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LT.{}
D;JLT
D=0
@SP
A=M
M=D
@SP
M=M+1
@CONTINUE.{}
0;JMP
(LT.{})
D=0
D=D-1
@SP
A=M
M=D
@SP
M=M+1
(CONTINUE.{})
",
                    self.label_count, self.label_count, self.label_count, self.label_count
                ))?)
            }
            "and" => Ok(self.file.write_fmt(format_args!(
                "@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M&D
@SP
M=M+1
"
            ))?),
            "or" => Ok(self.file.write_fmt(format_args!(
                "@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M|D
@SP
M=M+1
"
            ))?),
            "not" => Ok(self.file.write_fmt(format_args!(
                "@SP
M=M-1
A=M
M=!M
@SP
M=M+1
"
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
M=M+1
",
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
                    "@{}.{}
                    D=M
                    @SP
                    A=M
                    M=D
                    @SP
                    M=M+1 
                    ",
                    self.file_name, index
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
                    "@SP
                    M=M-1
                    A=M
                    D=M
                    @{}.{}
                    M=D
                    ",
                    self.file_name, index
                ))?),
                _ => Err(anyhow!("Invalid segment: {}", segment)),
            },
            _ => Err(anyhow!("Invalid command: {:?}", command)),
        }
    }

    pub fn write_init() {}

    pub fn write_label(&mut self, label: &str) -> Result<()> {
        Ok(self.file.write_fmt(format_args!(
            "({})
            ",
            label
        ))?)
    }

    pub fn write_goto(&mut self, label: &str) -> Result<()> {
        Ok(())
    }

    pub fn write_if(&mut self, label: &str) -> Result<()> {
        Ok(())
    }

    pub fn write_call(&mut self, function_name: &str, num_args: u32) -> Result<()> {
        Ok(())
    }

    pub fn write_return(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: u32) -> Result<()> {
        Ok(())
    }
}
