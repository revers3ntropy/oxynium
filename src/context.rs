use std::collections::{HashMap};
use crate::ast::ANON_DATA_PREFIX;

pub struct Context {
    pub declarations: HashMap<String, String>,
    symbols: Vec<String>,
    anon_symbol_count: u64,
    pub exec_mode: bool
}

impl Context {
    pub fn new() -> Context {
        Context {
            declarations: HashMap::new(),
            symbols: Vec::new(),
            anon_symbol_count: 0,
            exec_mode: false
        }
    }

    pub fn reserve_symbol(&mut self, symbol: String) -> String {
        self.symbols.push(symbol.clone());
        symbol
    }

    pub fn reserve_anon_symbol(&mut self) -> String {
        let symbol = format!("{}{}", ANON_DATA_PREFIX, self.anon_symbol_count);
        self.anon_symbol_count += 1;
        self.reserve_symbol(symbol)
    }

    pub fn declare(&mut self, ty: String) -> String {
        let name = self.reserve_anon_symbol();
        self.declarations.insert(name.clone(), ty);
        name
    }

    pub fn declare_symbol(&mut self, symbol: String, ty: String) {
        self.declarations.insert(symbol, ty);
    }
}