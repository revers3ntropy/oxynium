use crate::args::{Args, ExecMode};
use crate::ast::ANON_PREFIX;
use crate::context::{CallStackFrame, Context};
use crate::error::Error;
use crate::position::Interval;
use crate::symbols::{SymbolDec, SymbolDef};
use crate::types::Type;
use crate::util::{
    new_mut_rc, string_to_static_str, MutRc,
};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub struct ModuleContext {
    parent: MutRc<dyn Context>,
    file_path: String,
}

impl ModuleContext {
    pub fn new(
        parent: MutRc<dyn Context>,
        file_path: String,
    ) -> MutRc<Self> {
        new_mut_rc(Self { parent, file_path })
    }
}

impl Context for ModuleContext {
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
        let mut last = self_.clone();
        while let Some(parent) =
            self_.clone().borrow().get_parent()
        {
            last = self_.clone();
            self_ = parent;
        }
        last
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
        _path: &'static Path,
    ) {
        unreachable!()
    }

    fn get_current_dir_path(&self) -> &'static Path {
        let file_path_leaked_str = unsafe {
            string_to_static_str(self.file_path.clone())
        };
        Path::new(file_path_leaked_str).parent().unwrap()
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

    fn set_dec_as_defined(
        &mut self,
        _id: &str,
        _trace_interval: Interval,
    ) -> Result<(), Error> {
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

    fn define(
        &mut self,
        _symbol: SymbolDef,
        _trace_interval: Interval,
    ) -> Result<(), Error> {
        unreachable!()
    }

    fn define_global(
        &mut self,
        _symbol: SymbolDef,
        _trace_interval: Interval,
    ) -> Result<(), Error> {
        unreachable!()
    }

    fn get_definitions(
        &self,
    ) -> (Vec<&SymbolDef>, Vec<&SymbolDef>) {
        unreachable!()
    }

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

    fn str(&self) -> String {
        format!(
            "--- Module '{}' {}{}{}---",
            self.file_path,
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
        let file_name =
            Path::new(&self.file_path).file_stem().unwrap();
        self.parent.borrow().abs_module_path()
            + file_name.to_str().unwrap()
    }
}
