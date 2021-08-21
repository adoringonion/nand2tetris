use std::collections::HashMap;

pub struct SymbolTable {
    pub table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut symbol_table = Self {
            table: HashMap::new(),
        };

        symbol_table.add_entry("SP", 0);
        symbol_table.add_entry("LCL", 1);
        symbol_table.add_entry("ARG", 2);
        symbol_table.add_entry("THIS", 3);
        symbol_table.add_entry("THAT", 4);
        symbol_table.add_entry("R0", 0);
        symbol_table.add_entry("R1", 1);
        symbol_table.add_entry("R2", 2);
        symbol_table.add_entry("R3", 3);
        symbol_table.add_entry("R4", 4);
        symbol_table.add_entry("R5", 5);
        symbol_table.add_entry("R6", 6);
        symbol_table.add_entry("R7", 7);
        symbol_table.add_entry("R8", 8);
        symbol_table.add_entry("R9", 9);
        symbol_table.add_entry("R10", 10);
        symbol_table.add_entry("R11", 11);
        symbol_table.add_entry("R12", 12);
        symbol_table.add_entry("R13", 13);
        symbol_table.add_entry("R14", 14);
        symbol_table.add_entry("R15", 15);
        symbol_table.add_entry("SCREEN", 16384);
        symbol_table.add_entry("KBD", 24576);

        return  symbol_table;
    }

    pub fn add_entry(&mut self, symbol: &str, value: u16) {
        self.table.insert(symbol.to_string(), value);
    }

    pub fn contains(&self, name: &String) -> bool {
        self.table.contains_key(name)
    }

    pub fn get_address(&self, name: &String) -> Option<&u16> {
        self.table.get(name)
    }
}
