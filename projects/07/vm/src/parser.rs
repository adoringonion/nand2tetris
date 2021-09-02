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

pub struct Parser {}

impl Parser {
    fn new() -> Self {
        Parser {}
    }

    fn has_more_commands() -> bool {
        false
    }

    fn advance() {}

    fn arg1() -> String {
        String::from("")
    }

    fn arg2() -> u32 {
        0
    }
}
