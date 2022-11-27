use std::collections::{HashMap};
use crate::ast::ANON_PREFIX;
use crate::ast::types::Type;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub data: Option<String>,
    pub constant: bool,
    pub type_: Box<Type>
}

pub struct Context {
    // all global_vars are also in declarations
    declarations: HashMap<String, Symbol>,
    global_vars: Vec<String>,
    loop_label_stack: Vec<(String, String)>,
    anon_symbol_count: u64,
    pub exec_mode: u8
}

impl Context {
    pub fn new() -> Context {
        Context {
            declarations: HashMap::new(),
            global_vars: Vec::new(),
            loop_label_stack: Vec::new(),
            anon_symbol_count: 0,
            exec_mode: 0
        }
    }

    pub fn get_anon_id(&mut self) -> String {
        let symbol = format!("{}D{}", ANON_PREFIX, self.anon_symbol_count);
        self.anon_symbol_count += 1;
        symbol
    }

    pub fn get_anon_label(&mut self) -> String {
        let symbol = format!("{}L{}", ANON_PREFIX, self.anon_symbol_count);
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

    pub fn has_with_id(&self, id: &str) -> bool {
        self.declarations.get(id).is_some()
    }

    pub fn get_from_id(&self, id: &str) -> &Symbol {
        self.declarations.get(id).unwrap()
    }

    pub fn get_all_ids(&self) -> Vec<String> {
        self.declarations.keys().map(|s| s.to_string()).collect()
    }

    pub fn declare_anon_data(&mut self, data: String, constant: bool, type_: Box<Type>) -> String {
        let name = self.get_anon_id();
        let symbol = Symbol {
            name: name.clone(),
            data: Some(data),
            constant,
            type_
        };
        self.global_vars.push(name.clone());
        self.declare(symbol);
        name.clone()
    }

    pub fn get_global_vars(&mut self) -> Vec<&Symbol> {
        let mut symbols = Vec::new();
        for name in &self.global_vars {
            symbols.push(self.declarations.get(name).unwrap());
        }
        symbols
    }

    pub fn loop_labels_push(&mut self, start: String, end: String) {
        self.loop_label_stack.push((start, end));
    }

    pub fn loop_labels_pop(&mut self) -> (String, String) {
        if let Some(lbl) = self.loop_label_stack.pop() {
            lbl
        } else {
            panic!("Tried to pop from empty loop label stack");
        }
    }

    pub fn loop_labels_peak(&self) -> Option<(String, String)> {
        if let Some(lbl) = self.loop_label_stack.last() {
            Some(lbl.clone())
        } else {
            None
        }
    }
}