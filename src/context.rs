use std::collections::{HashMap};
use crate::ast::ANON_PREFIX;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub data: Option<String>,
    pub constant: bool,
}

pub struct Context {
    // all global_vars are also in declarations
    declarations: HashMap<String, Symbol>,
    global_vars: Vec<String>,
    anon_symbol_count: u64,
    pub exec_mode: u8
}

impl Context {
    pub fn new() -> Context {
        Context {
            declarations: HashMap::new(),
            global_vars: Vec::new(),
            anon_symbol_count: 0,
            exec_mode: 0
        }
    }

    pub fn get_anon_id(&mut self) -> String {
        let symbol = format!("{}{}", ANON_PREFIX, self.anon_symbol_count);
        self.anon_symbol_count += 1;
        symbol
    }

    pub fn declare(&mut self, symbol: Symbol) {
        self.declarations.insert(symbol.name.clone(), symbol);
    }

    pub fn declare_glob_var(&mut self, symbol: Symbol) {
        let name = symbol.name.clone();
        self.global_vars.push(name.clone());
        self.declarations.insert(name.clone(), symbol);
    }

    pub fn has_id(&self, id: &str) -> bool {
        self.declarations.contains_key(id)
    }

    pub fn get_global_vars(&mut self) -> Vec<&Symbol> {
        let mut symbols = Vec::new();
        for name in &self.global_vars {
            symbols.push(self.declarations.get(name).unwrap());
        }
        symbols
    }

    pub fn declare_anon_data(&mut self, data: String, constant: bool) -> String {
        let name = self.get_anon_id();
        let symbol = Symbol {
            name: name.clone(),
            data: Some(data),
            constant
        };
        self.global_vars.push(name.clone());
        self.declare(symbol);
        name.clone()
    }
}