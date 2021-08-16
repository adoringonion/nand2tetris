use parser::Parser;

use crate::parser::CommandType;

mod parser;

fn main() {
    let mut vec: Vec<String> = vec![];
    let mut parser = Parser::new();

    parser.advance();
    parser.advance();

    while parser.has_more_commands() {
        println!("{:?}", parser);
        let command = parser.command_type();

        match command {
            CommandType::ACommand | CommandType::LCommand => {
                let symbol_numeric = parser.symbol();
                vec.push(symbol_numeric);
            }
            CommandType::CCommand => {
                let dest = parser.dest();
                let comp = parser.comp();
                let jump = parser.jump();
                vec.push(dest);
                vec.push(comp);
                vec.push(jump);
            }
        }
        parser.advance();
    }

    println!("------------\n{:?}", vec);
}
