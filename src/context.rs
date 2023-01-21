use crate::args::{Args, ExecMode};
use crate::ast::ANON_PREFIX;
use crate::error::{type_error, Error};
use crate::position::Interval;
use crate::symbols::{SymbolDec, SymbolDef};
use crate::types::Type;
use crate::util::{indent, new_mut_rc, MutRc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
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
    pub exec_mode: ExecMode,
    pub std_asm_path: String,
    allow_overrides: bool,
    pub allow_local_var_decls: bool,
    // do not allow any more declarations
    frozen: bool,
    // throw error on unknown types
    err_on_unknowns: bool,
    pub cli_args: Args,
    concrete_type_cache: HashMap<String, MutRc<dyn Type>>,
    ignore_definitions: bool,
    current_dir_path: Option<&'static Path>,
}

impl Context {
    pub fn new(cli_args: Args) -> MutRc<Context> {
        new_mut_rc(Context {
            parent: None,
            declarations: HashMap::new(),
            definitions: HashMap::new(),
            loop_label_stack: Vec::new(),
            call_stack: Vec::new(),
            anon_symbol_count: 0,
            exec_mode: ExecMode::Bin,
            std_asm_path: String::from("std.asm"),
            allow_overrides: cli_args.allow_overrides,
            allow_local_var_decls: false,
            frozen: false,
            err_on_unknowns: false,
            cli_args,
            concrete_type_cache: HashMap::new(),
            ignore_definitions: false,
            current_dir_path: None,
        })
    }

    pub fn freeze(&mut self) {
        self.frozen = true;
    }
    pub fn reset(&mut self) {
        self.frozen = false;
        self.err_on_unknowns = false;
        self.set_ignoring_definitions(false);
    }
    pub fn is_frozen(&self) -> bool {
        if let Some(ref parent) = self.parent {
            parent.borrow().is_frozen()
        } else {
            self.frozen
        }
    }
    pub fn throw_on_unknowns(&self) -> bool {
        if let Some(ref parent) = self.parent {
            parent.borrow().throw_on_unknowns()
        } else {
            self.err_on_unknowns
        }
    }

    pub fn finished_resolving_types(&mut self) {
        self.freeze();
        self.err_on_unknowns = true;
    }

    pub fn set_parent(
        &mut self,
        parent: Rc<RefCell<Context>>,
    ) {
        self.exec_mode = parent.borrow_mut().exec_mode;
        self.std_asm_path =
            parent.borrow_mut().std_asm_path.clone();
        self.allow_overrides =
            parent.borrow_mut().allow_overrides;
        self.parent = Some(parent);
    }

    pub fn get_parent(&self) -> Option<MutRc<Context>> {
        self.parent.clone()
    }

    pub fn with_root<T>(
        &self,
        cb: &mut impl Fn(&Context) -> T,
    ) -> T {
        if let Some(ref parent) = self.parent {
            parent.borrow_mut().with_root(cb)
        } else {
            cb(self)
        }
    }

    pub fn with_root_mut<T>(
        &mut self,
        cb: &mut impl Fn(&mut Context) -> T,
    ) -> T {
        if let Some(ref parent) = self.parent {
            parent.borrow_mut().with_root_mut(cb)
        } else {
            cb(self)
        }
    }

    // Current Directory
    pub fn set_current_dir_path(
        &mut self,
        path: &'static Path,
    ) {
        assert!(self.current_dir_path.is_none());
        self.current_dir_path = Some(path);
    }
    pub fn get_current_dir_path(&self) -> &'static Path {
        if let Some(path) = self.current_dir_path {
            path
        } else if let Some(ref parent) = self.parent {
            parent.borrow().get_current_dir_path()
        } else {
            panic!("No current directory path set");
        }
    }

    // Generate unique identifiers (use root context)

    pub fn get_id(&mut self) -> usize {
        if self.parent.is_some() {
            return self
                .with_root_mut(&mut |ctx| ctx.get_id());
        }
        self.anon_symbol_count += 1;
        self.anon_symbol_count as usize
    }

    pub fn get_anon_label(&mut self) -> String {
        format!(".{}", self.get_global_anon_label())
    }
    pub fn get_global_anon_label(&mut self) -> String {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.get_global_anon_label()
            });
        }
        format!("{}L{}", ANON_PREFIX, self.get_id())
    }

    // Declarations

    pub fn declare(
        &mut self,
        symbol: SymbolDec,
        trace_interval: Interval,
    ) -> Result<SymbolDec, Error> {
        if self.is_frozen() {
            if self.has_dec_with_id(&symbol.name) {
                return Ok(
                    self.get_dec_from_id(&symbol.name)
                );
            }
            panic!("(!?) Context is frozen and symbol doesn't exist yet!");
        }
        if self.parent.is_some()
            && !self.allow_local_var_decls
            && !symbol.is_type
        {
            return self
                .parent
                .as_ref()
                .unwrap()
                .borrow_mut()
                .declare(symbol, trace_interval);
        }
        if let Some(duplicate) = self
            .declarations
            .get(symbol.name.clone().as_str())
        {
            if !self.allow_overrides {
                return Err(type_error(format!(
                    "Symbol `{}` is already declared",
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
    pub fn has_dec_with_id(&self, id: &str) -> bool {
        if self.declarations.contains_key(id) {
            true
        } else if self.parent.is_some() {
            self.parent
                .as_ref()
                .unwrap()
                .borrow()
                .has_dec_with_id(id)
        } else {
            false
        }
    }
    pub fn get_dec_from_id(&self, id: &str) -> SymbolDec {
        if let Some(dec) = self.declarations.get(id) {
            dec.clone()
        } else if self.parent.is_some() {
            self.parent
                .as_ref()
                .unwrap()
                .borrow()
                .get_dec_from_id(id)
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
            let mut dec =
                self.declarations.get(id).unwrap().clone();
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
            Err(type_error(format!(
                "Symbol {id} is not declared"
            ))
            .set_interval(trace_interval))
        }
    }
    pub fn update_dec_type(
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
        } else if self.parent.is_some() {
            self.parent
                .as_ref()
                .unwrap()
                .borrow_mut()
                .update_dec_type(
                    id,
                    new_type,
                    trace_interval,
                )
        } else {
            Err(type_error(format!(
                "Symbol {id} is not declared"
            ))
            .set_interval(trace_interval))
        }
    }

    pub fn get_new_local_var_offset(&self) -> usize {
        if self.allow_local_var_decls
            || self.parent.is_none()
        {
            let idx = self
                .declarations
                .iter()
                .filter(|d| !d.1.is_param)
                .count();
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
        if self.is_ignoring_definitions() {
            return Ok(());
        }
        if self.parent.is_some()
            && !self.allow_local_var_decls
        {
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

        self.definitions.insert(name.clone(), symbol);
        Ok(())
    }
    pub fn define_global(
        &mut self,
        symbol: SymbolDef,
        trace_interval: Interval,
    ) -> Result<(), Error> {
        if self.is_ignoring_definitions() {
            return Ok(());
        }
        if self.parent.is_some() {
            return self
                .parent
                .as_ref()
                .unwrap()
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

    pub fn get_definitions(
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

    pub fn loop_labels_push(
        &mut self,
        start: String,
        end: String,
    ) {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.loop_labels_push(
                    start.clone(),
                    end.clone(),
                )
            });
        }
        self.loop_label_stack.push((start, end));
    }

    pub fn loop_labels_pop(
        &mut self,
    ) -> Option<(String, String)> {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.loop_labels_pop()
            });
        }
        self.loop_label_stack.pop()
    }

    pub fn loop_label_peak(
        &self,
    ) -> Option<(String, String)> {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| {
                ctx.loop_label_peak()
            });
        }
        self.loop_label_stack.last().cloned()
    }

    // Stack Frames

    pub fn stack_frame_push(
        &mut self,
        frame: CallStackFrame,
    ) {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.stack_frame_push(frame.clone())
            });
        }
        self.call_stack.push(frame);
    }

    pub fn stack_frame_pop(
        &mut self,
    ) -> Option<CallStackFrame> {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.stack_frame_pop()
            });
        }
        self.call_stack.pop()
    }
    pub fn stack_frame_peak(
        &self,
    ) -> Option<CallStackFrame> {
        if self.parent.is_some() {
            return self.with_root(&mut |ctx| {
                ctx.stack_frame_peak()
            });
        }
        self.call_stack.last().cloned()
    }

    // Utils

    #[allow(dead_code)]
    pub fn str(&self) -> String {
        let mut s = format!(
            "--- Context {}{}{}---",
            if self.is_frozen() { "(frozen) " } else { "" },
            if self.err_on_unknowns {
                "(do err) "
            } else {
                ""
            },
            if self.allow_overrides {
                "(overrides) "
            } else {
                ""
            }
        );
        if self.parent.is_some() {
            s = format!(
                "{}",
                indent(
                    self.parent
                        .clone()
                        .unwrap()
                        .borrow()
                        .str(),
                    4
                )
            );
        }
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

    pub fn concrete_type_cache_get(
        &self,
        id: String,
    ) -> Option<MutRc<dyn Type>> {
        if self.parent.is_some() {
            return self
                .parent
                .clone()
                .unwrap()
                .borrow()
                .concrete_type_cache_get(id);
        }
        self.concrete_type_cache.get(&id).map(|t| t.clone())
    }

    pub fn concrete_type_cache_set(
        &mut self,
        id: String,
        t: MutRc<dyn Type>,
    ) {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.concrete_type_cache_set(
                    id.clone(),
                    t.clone(),
                )
            });
        }
        if self.concrete_type_cache.contains_key(&id) {
            panic!("Type {} already exists in cache", id);
        }
        self.concrete_type_cache.insert(id, t);
    }

    pub fn clear_concrete_cache(&mut self) {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.clear_concrete_cache()
            });
        }
        self.concrete_type_cache.clear();
    }

    pub fn concrete_type_cache_remove(&mut self, id: &str) {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.concrete_type_cache_remove(id)
            });
        }
        self.concrete_type_cache.remove(id);
    }

    // ignoring definitions
    pub fn set_ignoring_definitions(
        &mut self,
        value: bool,
    ) {
        if self.parent.is_some() {
            return self.with_root_mut(&mut |ctx| {
                ctx.set_ignoring_definitions(value)
            });
        }
        self.ignore_definitions = value;
    }
    pub fn is_ignoring_definitions(&self) -> bool {
        if self.parent.is_some() {
            return self
                .parent
                .clone()
                .unwrap()
                .borrow()
                .is_ignoring_definitions();
        }
        self.ignore_definitions
    }
}
