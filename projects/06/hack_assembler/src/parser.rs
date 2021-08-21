use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    vec::IntoIter,
};
use anyhow::Result;

pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: IntoIter<String>,
    current_token: Option<String>,
    next_token: Option<String>,
}

impl Parser {
    pub fn new(file: File) -> Result<Parser> {


        let lines = BufReader::new(file).lines();
        let tokens = pick_out_tokens(lines);

        let mut parser = Self {
            tokens,
            current_token: None,
            next_token: None,
        };

        // 初期値を設定するため、二回advanceを呼び出す
        parser.advance();
        parser.advance();

        Ok(parser)
    }

    pub fn has_more_commands(&self) -> bool {
        if let None = self.next_token {
            return false;
        }

        return true;
    }

    pub fn advance(&mut self) {
        self.current_token = std::mem::replace(&mut self.next_token, self.tokens.next());
    }

    pub fn command_type(&self) -> CommandType {
        let token = self.current_token.as_ref().expect("expected a command");

        if is_a_command(&token) {
            CommandType::ACommand
        } else if is_l_command(&token) {
            CommandType::LCommand
        } else {
            CommandType::CCommand
        }
    }

    pub fn symbol(&self) -> String {
        let mut token = self
            .current_token
            .as_ref()
            .expect("expected a symbol")
            .to_string();

        if is_a_command(&token) {
            token.retain(|c| c != '@');
            token
        } else {
            token.retain(|c| c != '(');
            token.retain(|c| c != ')');
            token
        }
    }

    pub fn dest(&self) -> String {
        let current_token = self.current_token.as_ref().expect("expected a dest");
        if !current_token.contains("=") {
            return String::from("null");
        }

        let equal_index = current_token.find("=").unwrap();
        return current_token[..equal_index].to_string();
    }

    pub fn comp(&self) -> String {
        let current_token = self.current_token.as_ref().expect("expected a comp");
        if current_token.contains("=") && current_token.contains(";") {
            let equal_index = current_token.find("=").unwrap();
            let semi_index = current_token.find(";").unwrap();
            return current_token[equal_index+1..semi_index].to_string();
        } else if current_token.contains("=") {
            let equal_index = current_token.find("=").unwrap();
            return current_token[equal_index+1..].to_string();
        } else {
            let semi_index = current_token.find(";").unwrap();
            return current_token[..semi_index].to_string();
        }
    }

    pub fn jump(&self) -> String {
        let current_token = self.current_token.as_ref().expect("expected a dest");
        if !current_token.contains(";") {
            return String::from("null");
        }

        let semi_index = current_token.find(";").unwrap();
        return current_token[semi_index+1..].to_string();
    }
}

fn skip_line(string: &String) -> bool {
    string.starts_with("//") || string.is_empty()
}

fn is_a_command(line: &String) -> bool {
    line.starts_with("@")
}

fn is_l_command(line: &String) -> bool {
    line.starts_with("(")
}

fn pick_out_tokens(lines: Lines<BufReader<File>>) -> IntoIter<String> {
    let mut tokens = vec![];
    for wrapped_line in lines {
        let line = wrapped_line.unwrap();
        if skip_line(&line) {
            continue;
        }

        let split_comments: Vec<&str> = line.split("//").collect();
        let split_whitespace: Vec<&str> = split_comments[0].split_whitespace().collect();
        tokens.push(split_whitespace[0].to_string());
    }

    tokens.into_iter()
}
