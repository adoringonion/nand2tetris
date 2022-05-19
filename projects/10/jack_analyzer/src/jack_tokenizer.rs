use std::{vec::IntoIter, fs::File, mem::replace};

pub enum Token {
    KEYWORD(Keyword),
    SYMBOL(String),
    IDENTIFIER(String),
    INT_CONST(i32),
    STRING_CONST(String),
}
    
pub enum Keyword {
    CLASS,
    METHOD,
    FUNCTION,
    CONSTRUCTOR,
    INT,
    BOOLEAN,
    CHAR,
    VOID,
    VAR,
    STATIC,
    FIELD,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
    TRUE,
    FALSE,
    NULL,
    THIS,
}

pub struct JackTokenizer {
    lines: IntoIter<Vec<String>>,
    current_token: Option<Token>>,
    next_token: Option<Token>,
}

impl JackTokenizer {
    
    pub fn new(file: File) -> Self {
        
    }

    pub fn has_more_tokens(&self) -> bool {
        match self.next_token {
            Some(_) => true,
            None => false,
        }
    }

    pub fn advance(&mut self) {
        self.current_token = replace(&mut self.next_token, self.lines.next());
    }

    pub fn token_type(&self) -> Token {
        match self.current_token {
            Some(Token::KEYWORD(keyword)) => Token::KEYWORD(keyword),
            Some(Token::SYMBOL(symbol)) => Token::SYMBOL(symbol),
            Some(Token::IDENTIFIER(identifier)) => Token::IDENTIFIER(identifier),
            Some(Token::INT_CONST(int_const)) => Token::INT_CONST(int_const),
            Some(Token::STRING_CONST(string_const)) => Token::STRING_CONST(string_const),
            None => Token::EOF,
        }
    }

    pub fn keyword(&self) -> Keyword {
        match self.current_token {
            Some(Token::KEYWORD(keyword)) => keyword,
            _ => panic!("Not a keyword"),
        }
    }

    pub fn symbol(&self) -> String {
        match self.current_token {
            Some(Token::SYMBOL(symbol)) => symbol,
            _ => panic!("Not a symbol"),
        }
    }

    pub fn identifier(&self) -> String {
        match self.current_token {
            Some(Token::IDENTIFIER(identifier)) => identifier,
            _ => panic!("Not a identifier"),
        }
    }

    pub fn int_val(&self) -> i32 {
        match self.current_token {
            Some(Token::INT_CONST(int_const)) => int_const,
            _ => panic!("Not a int_const"),
        }
    }




}