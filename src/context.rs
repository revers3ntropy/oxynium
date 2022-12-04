use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::ANON_PREFIX;
use crate::ast::types::Type;
use crate::error::{Error, type_error_unstructured};

pub type Ctx = Rc<RefCell<Context>>;

#[derive(Debug, Clone)]
pub struct SymbolDec {
    pub name: String,
    pub is_constant: bool,
    pub is_type: bool,
    pub type_: Box<Type>
}

impl SymbolDec {
    fn contains(&self, s: &SymbolDec) -> bool {
        self.type_.contains(&s.type_)
            && self.name == s.name
            && self.is_constant == s.is_constant
            && self.is_type == s.is_type
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolDef {
    pub name: String,
    pub data: Option<String>,
    pub text: Option<String>,
    pub is_local: bool,
}

#[derive(Debug)]
pub struct Context {
    parent: Option<Ctx>,
    // Declarations are analysed at compile time, and are used to type check
    declarations: HashMap<String, SymbolDec>,
    // Definitions are generated at compile time
    // and are inserted into the final assembly file
    definitions: HashMap<String, SymbolDef>,
    // Vec<(start of loop label, end of loop label)>
    loop_label_stack: Vec<(String, String)>,
    anon_symbol_count: u64,
    type_id_count: u64,
    pub exec_mode: u8,
    pub std_asm_path: String,
    pub allow_overrides: bool,
    pub id: u64
}

impl Context {
    pub fn new(parent: Option<Ctx>) -> Context {
        Context {
            parent,
            declarations: HashMap::new(),
            definitions: HashMap::new(),
            loop_label_stack: Vec::new(),
            anon_symbol_count: 0,
            exec_mode: 0,
            type_id_count: 100,
            std_asm_path: String::from("std.asm"),
            allow_overrides: false,
            // random id
            id: rand::random::<u64>()
        }
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<Context>>) {
        self.parent = Some(parent);
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn with_root<T>(&mut self, cb: &mut impl Fn(&mut Context) -> T) -> T {
        if self.parent.is_some() {
            let ref_ = self.parent.take().unwrap();
            let res = Rc::clone(&ref_).borrow_mut().with_root(cb);
            self.parent = Some(ref_);
            res
        } else {
            cb(self)
        }
    }

    // Generate unique identifiers

    pub fn get_anon_id(&mut self) -> String {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.get_anon_id());
        }
        let symbol = format!("{}D{}", ANON_PREFIX, self.anon_symbol_count);
        self.anon_symbol_count += 1;
        symbol
    }
    pub fn get_anon_label(&mut self) -> String {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.get_anon_label());
        }
        let symbol = format!("{}L{}", ANON_PREFIX, self.anon_symbol_count);
        self.anon_symbol_count += 1;
        symbol
    }
    pub fn get_type_id(&mut self) -> u64 {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.get_type_id());
        }
        self.type_id_count += 1;
        self.type_id_count
    }


    // Declarations

    pub fn declare(&mut self, symbol: SymbolDec) -> Result<(), Error> {
        if let Some(duplicate) = self.declarations.get(symbol.name.clone().as_str()) {
            if !self.allow_overrides || !duplicate.contains(&symbol) {
                return Err(type_error_unstructured(format!("Symbol {} is already declared", symbol.name)))
            }
        }
        self.declarations.insert(symbol.name.clone(), symbol);
        Ok(())
    }
    pub fn has_dec_with_id(&mut self, id: &str) -> bool {
        if self.declarations.get(id).is_some() {
            true
        } else if self.parent.is_some() {
            self.with_root(&mut |ctx| ctx.has_dec_with_id(id))
        } else {
            false
        }
    }
    pub fn get_dec_from_id(&mut self, id: &str) -> Result<SymbolDec, Error> {
        if self.declarations.get(id).is_some() {
            Ok(self.declarations.get(id).unwrap().clone())
        } else if self.parent.is_some() {
            self.with_root(&mut |ctx| ctx.get_dec_from_id(id))
        } else {
            Err(type_error_unstructured(format!("Symbol {} is not declared", id)))
        }
    }


    // Definitions

    pub fn define(&mut self, symbol: SymbolDef, anon: bool) -> Result<(), Error> {
        if self.definitions.get(symbol.name.clone().as_str()).is_some() {
            return Err(type_error_unstructured(format!("Symbol {} is already defined", symbol.name)))
        }
        if !anon && !self.declarations.get(symbol.name.clone().as_str()).is_some() {
            return Err(type_error_unstructured(format!("Symbol {} is not declared", symbol.name)))
        }
        let name = symbol.name.clone();
        self.definitions.insert(name.clone(), symbol);
        Ok(())
    }

    pub fn get_global_definitions(&mut self) -> (Vec<&SymbolDef>, Vec<&SymbolDef>) {
        let mut data = Vec::new();
        let mut text = Vec::new();
        for (_id, def) in self.definitions.iter() {
            if def.is_local {
                continue;
            }
            if def.data.is_some() {
                data.push(def);
            } else if def.text.is_some() {
                text.push(def);
            }
        }
        (data, text)
    }


    // Loop labels

    pub fn loop_labels_push(&mut self, start: String, end: String) {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| {
                ctx.loop_labels_push(start.clone(), end.clone())
            });
        }
        self.loop_label_stack.push((start, end));
    }

    pub fn loop_labels_pop(&mut self) -> (String, String) {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.loop_labels_pop());
        }
        if let Some(lbl) = self.loop_label_stack.pop() {
            lbl
        } else {
            panic!("Tried to pop from empty loop label stack");
        }
    }

    pub fn loop_labels_peak(&mut self) -> Option<(String, String)> {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.loop_labels_peak());
        }
        if let Some(lbl) = self.loop_label_stack.last() {
            Some(lbl.clone())
        } else {
            None
        }
    }
}