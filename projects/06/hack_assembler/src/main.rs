use parser::{CommandType, Parser};

mod code;
mod parser;

fn main() {
    let mut vec: Vec<String> = vec![];
    let mut parser = Parser::new();

    parser.advance();
    parser.advance();

    loop {
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
                println!("{}", code::dest(&dest));
            }
        }

        if !parser.has_more_commands() {
            break;
        }
        
        parser.advance();
    }

    println!("------------\n{:?}", vec);
}
