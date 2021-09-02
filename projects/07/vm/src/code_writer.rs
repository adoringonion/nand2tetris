use crate::parser::CommandType;

pub struct CodeWriter {}

impl CodeWriter {
    fn new() -> Self {
        CodeWriter {}
    }

    fn set_file_name(file_name: String) {}

    fn write_arithmetic(command: String) -> String {
        String::from("")
    }

    fn write_push_pop(command: CommandType, segment: String, index: u32) {}
}
