use crate::args::{Args, ExecMode};
use crate::ast::ANON_PREFIX;
use crate::context::{CallStackFrame, Context, LoopLabels};
use crate::error::Error;
use crate::position::Interval;
use crate::symbols::{SymbolDec, SymbolDef};
use crate::target::Target;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::rc::Rc;

pub struct RootContext {
    self_: Option<MutRc<dyn Context>>,
    // Vec<(start of loop label, end of loop label)>
    loop_label_stack: Vec<LoopLabels>,
    call_stack: Vec<CallStackFrame>,
    anon_symbol_count: u64,
    pub exec_mode: ExecMode,
    pub std_asm_path: &'static str,
    allow_overrides: bool,
    // do not allow any more declarations
    frozen: bool,
    // throw error on unknown types
    err_on_unknowns: bool,
    cli_args: Args,
    concrete_type_cache: HashMap<String, MutRc<dyn Type>>,
    ignore_definitions: bool,
    include_asm_paths: Vec<String>,
}

impl RootContext {
    pub fn new(cli_args: Args) -> MutRc<Self> {
        let self_ = mut_rc(Self {
            self_: None,
            loop_label_stack: vec![],
            call_stack: vec![],
            anon_symbol_count: 0,
            exec_mode: cli_args.exec_mode,
            std_asm_path: cli_args.std_path,
            allow_overrides: cli_args.allow_overrides,
            frozen: false,
            err_on_unknowns: false,
            cli_args,
            concrete_type_cache: HashMap::new(),
            ignore_definitions: false,
            include_asm_paths: vec![],
        });
        self_.borrow_mut().self_ = Some(self_.clone());
        self_
    }
}

impl Debug for RootContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Context for RootContext {
    fn reset(&mut self) {
        self.frozen = false;
        self.err_on_unknowns = false;
        self.set_ignoring_definitions(false);
    }

    fn freeze(&mut self) {
        self.frozen = true;
    }

    fn is_frozen(&self) -> bool {
        self.frozen
    }

    fn throw_on_unknowns(&self) -> bool {
        self.err_on_unknowns
    }

    fn finished_resolving_types(&mut self) {
        self.freeze();
        self.err_on_unknowns = true;
    }

    fn set_parent(&mut self, _parent: Rc<RefCell<dyn Context>>) {
        unreachable!()
    }

    fn get_parent(&self) -> Option<MutRc<dyn Context>> {
        None
    }

    fn global_scope(&self) -> MutRc<dyn Context> {
        unreachable!("RootContext::global_scope")
    }

    fn get_cli_args(&self) -> Args {
        self.cli_args.clone()
    }
    fn exec_mode(&self) -> ExecMode {
        self.exec_mode
    }
    fn std_asm_path(&self) -> &'static str {
        self.std_asm_path
    }

    fn allow_overrides(&self) -> bool {
        self.allow_overrides
    }

    fn set_current_dir_path(&mut self, _path: &'static Path) {
        unreachable!()
    }

    fn get_current_dir_path(&self) -> &'static Path {
        unreachable!()
    }

    fn get_id(&mut self) -> usize {
        self.anon_symbol_count += 1;
        self.anon_symbol_count as usize
    }
    fn get_anon_label(&mut self) -> String {
        format!(".{}", self.get_global_anon_label())
    }

    fn get_global_anon_label(&mut self) -> String {
        format!("{}L{}", ANON_PREFIX, self.get_id())
    }

    fn declare(
        &mut self,
        _symbol: SymbolDec,
        _trace_interval: Interval,
    ) -> Result<SymbolDec, Error> {
        unreachable!()
    }

    fn has_dec_with_id(&self, _id: &str) -> bool {
        false
    }

    fn get_dec_from_id(&self, _id: &str) -> SymbolDec {
        unreachable!()
    }

    fn set_dec_as_defined(&mut self, _id: &str, _trace_interval: Interval) -> Result<(), Error> {
        unreachable!()
    }

    fn update_dec_type(
        &mut self,
        _id: &str,
        _new_type: MutRc<dyn Type>,
        _trace_interval: Interval,
    ) -> Result<(), Error> {
        unreachable!()
    }

    fn get_new_local_var_offset(&self) -> usize {
        unreachable!()
    }

    fn define(&mut self, _symbol: SymbolDef, _trace_interval: Interval) -> Result<(), Error> {
        unreachable!()
    }

    fn define_global(
        &mut self,
        _symbol: SymbolDef,
        _trace_interval: Interval,
    ) -> Result<(), Error> {
        unreachable!()
    }

    fn get_definitions(&self) -> (Vec<&SymbolDef>, Vec<&SymbolDef>) {
        unreachable!()
    }

    fn loop_labels_push(&mut self, lbl: LoopLabels) {
        self.loop_label_stack.push(lbl);
    }

    fn loop_labels_pop(&mut self) -> Option<LoopLabels> {
        self.loop_label_stack.pop()
    }

    fn loop_label_peak(&self) -> Option<LoopLabels> {
        self.loop_label_stack.last().cloned()
    }

    fn stack_frame_push(&mut self, frame: CallStackFrame) {
        self.call_stack.push(frame);
    }

    fn stack_frame_pop(&mut self) -> Option<CallStackFrame> {
        self.call_stack.pop()
    }

    fn stack_frame_peak(&self) -> Option<CallStackFrame> {
        self.call_stack.last().cloned()
    }

    fn str(&self) -> String {
        format!(
            "--- Root Ctx {}{}{}---",
            if self.is_frozen() { "(frozen) " } else { "" },
            if self.throw_on_unknowns() {
                "(do err) "
            } else {
                ""
            },
            if self.allow_overrides() {
                "(overrides) "
            } else {
                ""
            },
        )
    }

    fn concrete_type_cache_get(&self, id: String) -> Option<MutRc<dyn Type>> {
        self.concrete_type_cache.get(&id).map(|t| t.clone())
    }

    fn concrete_type_cache_set(&mut self, id: String, t: MutRc<dyn Type>) {
        if self.concrete_type_cache.contains_key(&id) {
            panic!("Type {} already exists in cache", id);
        }
        self.concrete_type_cache.insert(id, t);
    }

    fn clear_concrete_cache(&mut self) {
        self.concrete_type_cache.clear();
    }

    fn concrete_type_cache_remove(&mut self, id: &str) {
        self.concrete_type_cache.remove(id);
    }

    fn set_ignoring_definitions(&mut self, value: bool) {
        self.ignore_definitions = value;
    }

    fn is_ignoring_definitions(&self) -> bool {
        self.ignore_definitions
    }

    fn target(&self) -> Target {
        self.cli_args.target.clone()
    }

    fn include_asm(&mut self, asm_path: String) {
        self.include_asm_paths.push(asm_path);
    }

    fn get_included_asm_paths(&self) -> Vec<String> {
        self.include_asm_paths.clone()
    }
}
