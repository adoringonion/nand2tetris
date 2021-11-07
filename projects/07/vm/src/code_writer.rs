use anyhow::{anyhow, Result};
use std::{fs::File, io::Write};

use crate::parser::CommandType;

pub struct CodeWriter {
    file_name: String,
    pub file: File,
    label_count: usize,
    current_function: Option<String>,
}

impl CodeWriter {
    pub fn new(file_name: &str) -> Self {
        let file = File::create(format!("{}.asm", file_name)).unwrap();
        CodeWriter {
            file_name: file_name.to_string(),
            file,
            label_count: 0,
            current_function: None,
        }
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
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

    pub fn write_init(&mut self) -> Result<()> {
        self.file.write_fmt(format_args!(
            "@256
            D=A
            @SP
            M=D
            "
        ))?;
        self.write_call("Sys.init", 0)?;
        Ok(())
    }

    pub fn write_label(&mut self, label: &str) -> Result<()> {
        match &self.current_function {
            Some(function) => {
                Ok(self.file.write_fmt(format_args!(
                    "({}${})
                    ",
                    function, label
                ))?)
            }
            None => {Ok(self.file.write_fmt(format_args!(
                "({})
                ",
                label
            ))?)},
        }
    }

    pub fn write_goto(&mut self, label: &str) -> Result<()> {
        match &self.current_function {
            Some(function) => Ok(self.file.write_fmt(format_args!(
                "@{}${}
                0;JMP
                ",
                function, label
            ))?),
            None => Ok(self.file.write_fmt(format_args!(
                "@{}
                0;JMP
                ",
                label
            ))?),
        }
    }

    pub fn write_if(&mut self, label: &str) -> Result<()> {
        match &self.current_function {
            Some(function) => Ok(self.file.write_fmt(format_args!(
                "@SP
                AM=M-1
                D=M
                @{}${}
                D;JNE
                ",
                function, label
            ))?),
            None => Ok(self.file.write_fmt(format_args!(
                "@SP
                AM=M-1
                D=M
                @{}
                D;JNE
                ",
                label
            ))?),
        }
    }

    pub fn write_call(&mut self, function_name: &str, num_args: u32) -> Result<()> {
        self.label_count += 1;
        Ok(self.file.write_fmt(format_args!(
            "@RETURN.{}
            D=A
            @SP
            A=M
            M=D
            @SP
            M=M+1

            @LCL
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1

            @ARG
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1

            @THIS
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1

            @THAT
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1

            @SP
            D=M
            @5
            D=D-A
            @{}
            D=D-A
            @ARG
            M=D
            @SP
            D=M
            @LCL
            M=D
            @{}
            0;JMP
            (RETURN.{})
            ",
            self.label_count, num_args, function_name, self.label_count
        ))?)
    }

    pub fn write_return(&mut self) -> Result<()> {
        Ok(self.file.write_fmt(format_args!(
            "@LCL
            D=M
            @R13
            M=D

            @5
            D=A
            @R13
            A=M-D
            D=M
            @R14
            M=D

            @SP
            AM=M-1
            D=M
            @ARG
            A=M
            M=D
            @ARG
            D=M+1
            @SP
            M=D

            @1
            D=A
            @R13
            A=M-D
            D=M
            @THAT
            M=D
            
            @2
            D=A
            @R13
            A=M-D
            D=M
            @THIS
            M=D

            @3
            D=A
            @R13
            A=M-D
            D=M
            @ARG
            M=D

            @4
            D=A
            @R13
            A=M-D
            D=M
            @LCL
            M=D

            @R14
            A=M
            0;JMP
            "
        ))?)
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: u32) -> Result<()> {
        self.label_count += 1;
        self.current_function = Some(function_name.to_string());
        Ok(self.file.write_fmt(format_args!(
            "({})
            @{}
            D=A
            @R13
            M=D
            (WHILE1.{})
            @R13
            D=M
            @WHILE2.{}
            D;JEQ
            @0
            D=A
            @SP
            A=M
            M=D
            @SP
            M=M+1
            @R13
            D=M
            @1
            D=D-A
            @R13
            M=D
            @WHILE1.{}
            0;JMP
            (WHILE2.{})
            ",
            function_name,
            num_locals,
            self.label_count,
            self.label_count,
            self.label_count,
            self.label_count
        ))?)
    }
}
