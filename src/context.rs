use crate::ast::ANON_PREFIX;
use crate::error::{type_error, Error};
use crate::position::Interval;
use crate::symbols::{SymbolDec, SymbolDef};
use crate::util::{new_mut_rc, MutRc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CallStackFrame {
    pub name: String,
    pub params: Vec<String>,
    pub ret_lbl: String,
}

#[derive(Debug)]
pub struct Context {
    parent: Option<MutRc<Context>>,
    // Declarations are analysed at compile time, and are used to type check
    declarations: HashMap<String, SymbolDec>,
    // Definitions are generated at compile time
    // and are inserted into the final assembly file
    definitions: HashMap<String, SymbolDef>,
    // Vec<(start of loop label, end of loop label)>
    loop_label_stack: Vec<(String, String)>,
    call_stack: Vec<CallStackFrame>,
    anon_symbol_count: u64,
    pub exec_mode: u8,
    pub std_asm_path: String,
    pub allow_overrides: bool,
    pub allow_local_var_decls: bool,
}

impl Context {
    pub fn new() -> MutRc<Context> {
        new_mut_rc(Context {
            parent: None,
            declarations: HashMap::new(),
            definitions: HashMap::new(),
            loop_label_stack: Vec::new(),
            call_stack: Vec::new(),
            anon_symbol_count: 0,
            exec_mode: 0,
            std_asm_path: String::from("std.asm"),
            allow_overrides: false,
            allow_local_var_decls: false,
        })
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<Context>>) {
        self.exec_mode = parent.borrow_mut().exec_mode;
        self.std_asm_path = parent.borrow_mut().std_asm_path.clone();
        self.allow_overrides = parent.borrow_mut().allow_overrides;
        self.parent = Some(parent);
    }

    pub fn with_root<T>(&mut self, cb: &mut impl Fn(&mut Context) -> T) -> T {
        if self.parent.is_some() {
            let ref_ = self.parent.take().unwrap();
            let res = ref_.clone().borrow_mut().with_root(cb);
            self.parent = Some(ref_);
            res
        } else {
            cb(self)
        }
    }

    // Generate unique identifiers (use root context)

    pub fn get_anon_label(&mut self) -> String {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.get_anon_label());
        }
        let symbol = format!("{}L{}", ANON_PREFIX, self.anon_symbol_count);
        self.anon_symbol_count += 1;
        symbol
    }

    // Declarations

    pub fn declare(
        &mut self,
        symbol: SymbolDec,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.parent.is_some() && !self.allow_local_var_decls {
            return self
                .parent
                .as_ref()
                .unwrap()
                .borrow_mut()
                .declare(symbol, trace_interval);
        }
        if let Some(duplicate) =
            self.declarations.get(symbol.name.clone().as_str())
        {
            if !self.allow_overrides || !duplicate.contains(&symbol) {
                return Err(type_error(format!(
                    "Symbol {} is already declared",
                    symbol.name
                ))
                .set_interval(trace_interval));
            }
        }
        self.declarations.insert(symbol.name.clone(), symbol);
        Ok(())
    }
    pub fn has_dec_with_id(&self, id: &str) -> bool {
        if self.declarations.get(id).is_some() {
            true
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().borrow().has_dec_with_id(id)
        } else {
            false
        }
    }
    pub fn get_dec_from_id(&self, id: &str) -> SymbolDec {
        if self.declarations.get(id).is_some() {
            self.declarations.get(id).unwrap().clone()
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().borrow().get_dec_from_id(id)
        } else {
            panic!("Symbol {} not found", id);
        }
    }
    pub fn set_dec_as_defined(
        &mut self,
        id: &str,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.declarations.get(id).is_some() {
            let mut dec = self.declarations.get(id).unwrap().clone();
            dec.is_defined = true;
            self.declarations.insert(id.to_string(), dec);
            Ok(())
        } else if self.parent.is_some() {
            self.parent
                .as_ref()
                .unwrap()
                .borrow_mut()
                .set_dec_as_defined(id, trace_interval)
        } else {
            Err(type_error(format!("Symbol {id} is not declared"))
                .set_interval(trace_interval))
        }
    }

    pub fn get_new_local_var_offset(&self) -> usize {
        if self.allow_local_var_decls || self.parent.is_none() {
            let idx =
                self.declarations.iter().filter(|d| !d.1.is_param).count();
            (1 + idx) * 8
        } else {
            self.parent
                .as_ref()
                .unwrap()
                .borrow()
                .get_new_local_var_offset()
        }
    }

    // Definitions

    pub fn define(
        &mut self,
        symbol: SymbolDef,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.parent.is_some() && !self.allow_local_var_decls {
            return self
                .parent
                .as_ref()
                .unwrap()
                .borrow_mut()
                .define(symbol, trace_interval);
        }
        let name = symbol.name.clone();
        if self.definitions.get(&name.clone()).is_some() {
            return Err(type_error(format!(
                "Symbol {} is already defined",
                symbol.name
            ))
            .set_interval(trace_interval));
        }
        if !self.declarations.get(&name.clone()).is_some() {
            return Err(type_error(format!(
                "Symbol {} is not declared",
                symbol.name
            ))
            .set_interval(trace_interval));
        }

        self.definitions.insert(name.clone(), symbol);
        Ok(())
    }
    pub fn define_anon(
        &mut self,
        mut symbol: SymbolDef,
        trace_interval: Interval,
    ) -> Result<String, Error> {
        if self.parent.is_some() {
            return self
                .parent
                .as_ref()
                .unwrap()
                .borrow_mut()
                .define_anon(symbol, trace_interval);
        }
        symbol.name = self.get_anon_label();
        if self.definitions.get(symbol.name.clone().as_str()).is_some() {
            return Err(type_error(format!(
                "Symbol {} is already defined",
                symbol.name
            ))
            .set_interval(trace_interval));
        }
        let name = symbol.name.clone();
        self.definitions.insert(name.clone(), symbol);
        Ok(name.clone())
    }
    pub fn get_def_from_id(&self, id: &str) -> Result<SymbolDef, Error> {
        if self.definitions.get(id).is_some() {
            Ok(self.definitions.get(id).unwrap().clone())
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().borrow().get_def_from_id(id)
        } else {
            Err(type_error(format!("Symbol {} is not defined", id)))
        }
    }
    pub fn get_definitions(&self) -> (Vec<&SymbolDef>, Vec<&SymbolDef>) {
        let mut data = Vec::new();
        let mut text = Vec::new();
        for (_id, def) in self.definitions.iter() {
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

    pub fn loop_labels_pop(&mut self) -> Option<(String, String)> {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.loop_labels_pop());
        }
        self.loop_label_stack.pop()
    }

    pub fn loop_label_peak(&mut self) -> Option<(String, String)> {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.loop_label_peak());
        }
        self.loop_label_stack.last().cloned()
    }

    // Stack Frames

    pub fn stack_frame_push(&mut self, frame: CallStackFrame) {
        if self.parent.is_some() {
            return self
                .with_root(&mut |ctx| ctx.stack_frame_push(frame.clone()));
        }
        self.call_stack.push(frame);
    }

    pub fn stack_frame_pop(&mut self) -> Option<CallStackFrame> {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.stack_frame_pop());
        }
        self.call_stack.pop()
    }
    pub fn stack_frame_peak(&mut self) -> Option<CallStackFrame> {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| ctx.stack_frame_peak());
        }
        self.call_stack.last().cloned()
    }
}
