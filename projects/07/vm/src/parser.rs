use std::{
    fs::File,
    io::{BufRead, BufReader},
    mem::replace,
    vec::IntoIter,
};

#[derive(Debug)]
pub enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    CLabel,
    CGoto,
    CIf,
    CFunction,
    CReturn,
    CCall,
}

pub struct Parser {
    lines: IntoIter<Vec<String>>,
    current_line: Option<Vec<String>>,
    next_line: Option<Vec<String>>,
}

impl Parser {
    pub fn new(file: File) -> Self {
        let lines = BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.starts_with("//"))
            .filter(|line| !line.is_empty())
            .map(remove_comments)
            .map(|line| line.split_whitespace().map(|x| x.to_string()).collect())
            .collect::<Vec<Vec<String>>>()
            .into_iter();

        let mut parser = Self {
            lines,
            current_line: None,
            next_line: None,
        };

        // call advance() twice to set the first token and the second token
        parser.advance();
        parser.advance();

        parser
    }

    pub fn has_more_commands(&self) -> bool {
        if let None = self.next_line {
            false
        } else {
            true
        }
    }

    pub fn advance(&mut self) {
        self.current_line = replace(&mut self.next_line, self.lines.next());
    }

    pub fn command_type(&self) -> CommandType {
        let command = self.current_line.as_ref().unwrap();

        match command[0].as_str() {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                CommandType::CArithmetic
            }
            "push" => CommandType::CPush,
            "pop" => CommandType::CPop,
            "label" => CommandType::CLabel,
            "goto" => CommandType::CGoto,
            "if-goto" => CommandType::CIf,
            "function" => CommandType::CFunction,
            "call" => CommandType::CCall,
            "return" => CommandType::CReturn,
            _ => panic!("Unknown command type: {}", command[0]),
        }
    }

    pub fn arg1(&self) -> String {
        let command = self.current_line.as_ref().unwrap();

        match self.command_type() {
            CommandType::CArithmetic => command[0].clone(),
            _ => command[1].clone(),
        }
    }

    pub fn arg2(&self) -> u32 {
        let command = self.current_line.as_ref().unwrap();

        return command[2].parse::<u32>().unwrap();
    }
}

fn remove_comments(line: String) -> String {
    if let Some(index) = line.find("//") {
        return line[..index].to_string();
    } else {
        return line;
    }
}
