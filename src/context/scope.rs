use crate::args::{Args, ExecMode};
use crate::context::{CallStackFrame, Context};
use crate::error::{type_error, Error};
use crate::position::Interval;
use crate::symbols::{SymbolDec, SymbolDef};
use crate::types::Type;
use crate::util::{indent, new_mut_rc, MutRc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub struct Scope {
    parent: MutRc<dyn Context>,
    // Declarations are analysed at compile time, and are used to type check
    declarations: HashMap<String, SymbolDec>,
    // Definitions are generated at compile time
    // and are inserted into the final assembly file
    definitions: HashMap<String, SymbolDef>,
    allow_local_var_decls: bool,
    current_dir_path: Option<&'static Path>,
    is_global: bool,
}

impl Scope {
    pub fn new(
        parent: MutRc<dyn Context>,
        allow_local_var_decls: bool,
        is_global: bool,
    ) -> MutRc<dyn Context> {
        new_mut_rc(Scope {
            parent,
            declarations: HashMap::new(),
            definitions: HashMap::new(),
            allow_local_var_decls,
            current_dir_path: None,
            is_global,
        })
    }
    pub fn new_local(
        parent: MutRc<dyn Context>,
    ) -> MutRc<dyn Context> {
        Scope::new(parent, false, false)
    }

    pub fn new_fn_ctx(
        parent: MutRc<dyn Context>,
    ) -> MutRc<dyn Context> {
        Scope::new(parent, true, false)
    }

    pub fn new_global(
        root: MutRc<dyn Context>,
    ) -> MutRc<dyn Context> {
        Scope::new(root, true, true)
    }
}

impl Context for Scope {
    fn reset(&mut self) {
        self.parent.borrow_mut().reset();
    }
    fn freeze(&mut self) {
        self.parent.borrow_mut().freeze();
    }
    fn is_frozen(&self) -> bool {
        self.parent.borrow().is_frozen()
    }
    fn throw_on_unknowns(&self) -> bool {
        self.parent.borrow().throw_on_unknowns()
    }
    fn finished_resolving_types(&mut self) {
        self.parent.borrow_mut().finished_resolving_types();
    }

    fn set_parent(
        &mut self,
        parent: Rc<RefCell<dyn Context>>,
    ) {
        self.parent = parent;
    }

    fn get_parent(&self) -> Option<MutRc<dyn Context>> {
        Some(self.parent.clone())
    }

    fn root(
        &self,
        _self: MutRc<dyn Context>,
    ) -> MutRc<dyn Context> {
        self.parent.clone()
    }

    fn global_scope(
        &self,
        mut self_: MutRc<dyn Context>,
    ) -> MutRc<dyn Context> {
        // root -> module -> scope -> scope, want last scope
        let mut last_scope = self_.clone();
        let mut module = self_.clone();
        while let Some(parent) =
            self_.clone().borrow().get_parent()
        {
            last_scope = module.clone();
            module = self_.clone();
            self_ = parent;
        }
        last_scope
    }

    fn get_cli_args(&self) -> Args {
        self.parent.borrow().get_cli_args()
    }

    fn exec_mode(&self) -> ExecMode {
        self.parent.borrow().exec_mode()
    }
    fn std_asm_path(&self) -> &'static str {
        self.parent.borrow().std_asm_path()
    }
    fn allow_overrides(&self) -> bool {
        self.parent.borrow().allow_overrides()
    }

    fn set_current_dir_path(
        &mut self,
        path: &'static Path,
    ) {
        assert!(self.current_dir_path.is_none());
        self.current_dir_path = Some(path);
    }
    fn get_current_dir_path(&self) -> &'static Path {
        if let Some(path) = self.current_dir_path {
            path
        } else {
            self.parent.borrow().get_current_dir_path()
        }
    }

    fn get_id(&mut self) -> usize {
        self.parent.borrow_mut().get_id()
    }

    fn get_anon_label(&mut self) -> String {
        format!(".{}", self.get_global_anon_label())
    }
    fn get_global_anon_label(&mut self) -> String {
        self.parent.borrow_mut().get_global_anon_label()
    }

    fn declare(
        &mut self,
        symbol: SymbolDec,
        trace_interval: Interval,
    ) -> Result<SymbolDec, Error> {
        if self.is_frozen() {
            if !symbol.is_type
                && self.has_dec_with_id(&symbol.name)
            {
                return Ok(
                    self.get_dec_from_id(&symbol.name)
                );
            }
            if !symbol.is_type {
                panic!("(!?) Context is frozen and symbol '{}' doesn't exist yet!", symbol.name);
            }
        }

        if !self.allow_local_var_decls && !symbol.is_type {
            return self
                .parent
                .borrow_mut()
                .declare(symbol, trace_interval);
        }
        if let Some(duplicate) = self
            .declarations
            .get(symbol.name.clone().as_str())
        {
            if !symbol.is_type && !self.allow_overrides() {
                return Err(type_error(format!(
                    "symbol `{}` is already declared",
                    symbol.name
                ))
                .set_interval(trace_interval));
            }
            if !duplicate.contains(&symbol) {
                return Err(type_error(format!(
                    "Symbol `{}` redeclared with incompatible type: expected `{}` but found `{}`",
                    symbol.name,
                    duplicate.type_.borrow().str(),
                    symbol.type_.borrow().str()
                ))
                    .set_interval(trace_interval));
            }
        }
        self.declarations
            .insert(symbol.name.clone(), symbol.clone());
        Ok(symbol)
    }
    fn has_dec_with_id(&self, id: &str) -> bool {
        if self.declarations.contains_key(id) {
            true
        } else {
            self.parent.borrow().has_dec_with_id(id)
        }
    }
    fn get_dec_from_id(&self, id: &str) -> SymbolDec {
        if let Some(dec) = self.declarations.get(id) {
            dec.clone()
        } else if !self.is_global {
            self.parent.borrow().get_dec_from_id(id)
        } else {
            panic!("Symbol '{}' not found!", id);
        }
    }
    fn set_dec_as_defined(
        &mut self,
        id: &str,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.declarations.get(id).is_some() {
            let mut dec =
                self.declarations.get(id).unwrap().clone();
            dec.is_defined = true;
            self.declarations.insert(id.to_string(), dec);
            Ok(())
        } else {
            self.parent
                .borrow_mut()
                .set_dec_as_defined(id, trace_interval)
        }
    }
    fn update_dec_type(
        &mut self,
        id: &str,
        new_type: MutRc<dyn Type>,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.declarations.get(id).is_some() {
            let mut dec =
                self.declarations.get(id).unwrap().clone();
            dec.type_ = new_type;
            self.declarations.insert(id.to_string(), dec);
            Ok(())
        } else {
            self.parent.borrow_mut().update_dec_type(
                id,
                new_type,
                trace_interval,
            )
        }
    }

    fn get_new_local_var_offset(&self) -> usize {
        if self.allow_local_var_decls {
            let idx = self
                .declarations
                .iter()
                .filter(|d| !d.1.is_param && !d.1.is_type)
                .count();
            (1 + idx) * 8
        } else {
            self.parent.borrow().get_new_local_var_offset()
        }
    }

    fn define(
        &mut self,
        symbol: SymbolDef,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.is_ignoring_definitions() {
            return Ok(());
        }
        if !self.allow_local_var_decls {
            return self
                .parent
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

        self.definitions.insert(name.clone(), symbol);
        Ok(())
    }
    fn define_global(
        &mut self,
        symbol: SymbolDef,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.is_ignoring_definitions() {
            return Ok(());
        }
        if !self.is_global {
            return self
                .parent
                .borrow_mut()
                .define_global(symbol, trace_interval);
        }
        if self
            .definitions
            .get(symbol.name.clone().as_str())
            .is_some()
        {
            return Err(type_error(format!(
                "Symbol {} is already defined",
                symbol.name
            ))
            .set_interval(trace_interval));
        }
        self.definitions
            .insert(symbol.name.clone(), symbol);
        Ok(())
    }

    fn get_definitions(
        &self,
    ) -> (Vec<&SymbolDef>, Vec<&SymbolDef>) {
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

    fn loop_labels_push(
        &mut self,
        start: String,
        end: String,
    ) {
        self.parent
            .borrow_mut()
            .loop_labels_push(start, end)
    }

    fn loop_labels_pop(
        &mut self,
    ) -> Option<(String, String)> {
        self.parent.borrow_mut().loop_labels_pop()
    }

    fn loop_label_peak(&self) -> Option<(String, String)> {
        self.parent.borrow().loop_label_peak()
    }

    // Stack Frames

    fn stack_frame_push(&mut self, frame: CallStackFrame) {
        self.parent.borrow_mut().stack_frame_push(frame)
    }

    fn stack_frame_pop(
        &mut self,
    ) -> Option<CallStackFrame> {
        self.parent.borrow_mut().stack_frame_pop()
    }
    fn stack_frame_peak(&self) -> Option<CallStackFrame> {
        self.parent.borrow().stack_frame_peak()
    }

    // Utils

    fn str(&self) -> String {
        let mut s = format!(
            "--- Scope ---\n{}",
            indent(self.parent.borrow().str(), 4)
        );
        for (_, dec) in self.declarations.iter() {
            s.push_str(&format!("\n  {}", dec.str()));
        }
        if self.declarations.is_empty() {
            s.push_str("\n  (Empty)");
        }
        s.push_str("\n----------------------------------");
        s
    }

    // Concrete Type Cache

    fn concrete_type_cache_get(
        &self,
        id: String,
    ) -> Option<MutRc<dyn Type>> {
        self.parent.borrow().concrete_type_cache_get(id)
    }

    fn concrete_type_cache_set(
        &mut self,
        id: String,
        t: MutRc<dyn Type>,
    ) {
        self.parent
            .borrow_mut()
            .concrete_type_cache_set(id, t)
    }

    fn clear_concrete_cache(&mut self) {
        self.parent.borrow_mut().clear_concrete_cache()
    }

    fn concrete_type_cache_remove(&mut self, id: &str) {
        self.parent
            .borrow_mut()
            .concrete_type_cache_remove(id)
    }

    // ignoring definitions
    fn set_ignoring_definitions(&mut self, value: bool) {
        self.parent
            .borrow_mut()
            .set_ignoring_definitions(value)
    }
    fn is_ignoring_definitions(&self) -> bool {
        self.parent.borrow().is_ignoring_definitions()
    }

    fn abs_module_path(&self) -> String {
        self.parent.borrow().abs_module_path()
    }
}
