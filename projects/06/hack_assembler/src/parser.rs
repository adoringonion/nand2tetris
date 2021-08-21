use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec::IntoIter,
};

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
        let tokens = BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.starts_with("//"))
            .filter(|line| !line.is_empty())
            .map(pick_out_tokens)
            .collect::<Vec<String>>()
            .into_iter();

        let mut parser = Self {
            tokens,
            current_token: None,
            next_token: None,
        };

        // call advance() twice to set the first token and the second token
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
        let token = self.current_token.as_ref().unwrap();

        if is_a_command(&token) {
            CommandType::ACommand
        } else if is_l_command(&token) {
            CommandType::LCommand
        } else {
            CommandType::CCommand
        }
    }

    pub fn symbol(&self) -> String {
        let mut token = self.current_token.as_ref().unwrap().to_string();

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
        let current_token = self.current_token.as_ref().unwrap();
        if let Some(index) = current_token.find("=") {
            return current_token[..index].to_string();
        }

        return String::from("null");
    }

    pub fn comp(&self) -> String {
        let current_token = self.current_token.as_ref().unwrap();
        match current_token.find("=") {
            Some(equal_index) => match current_token.find(";") {
                Some(semicolon_index) => {
                    current_token[equal_index+1..semicolon_index].to_string()
                }
                None => current_token[equal_index+1..].to_string(),
            },
            None => match current_token.find(";") {
                Some(semicolon_index) => current_token[..semicolon_index].to_string(),
                None => current_token.to_string(),
            },
        }
    }

    pub fn jump(&self) -> String {
        let current_token = self.current_token.as_ref().unwrap();
        if let Some(index) = current_token.find(";") {
            return current_token[..index].to_string();
        }

        return String::from("null");
    }
}

fn is_a_command(token: &String) -> bool {
    token.starts_with("@")
}

fn is_l_command(token: &String) -> bool {
    token.starts_with("(")
}

fn pick_out_tokens(mut line: String) -> String {
    line.retain(|c| !c.is_whitespace());
    if let Some(index) = line.find("//") {
        return line[..index].to_string();
    } else {
        return line;
    }
}
