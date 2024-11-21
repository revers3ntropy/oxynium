pub mod root_ctx;
pub mod scope;

use crate::args::{Args, ExecMode};
use crate::error::Error;
use crate::position::Interval;
use crate::symbols::{SymbolDec, SymbolDef};
use crate::target::Target;
use crate::types::Type;
use crate::util::MutRc;
use std::cell::RefCell;
use std::fmt::Debug;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct CallStackFrame {
    pub name: String,
    // pub params: Vec<String>,
    pub ret_lbl: String,
}

// used by break and continue statements to find the correct label to jump to
#[derive(Debug, Clone)]
pub struct LoopLabels {
    // the reason we need this label (and can't use the start of the loop)
    // is that in `for _ in range` loops, `continue` statements wouldn't be
    // able to skip the rest of the body and increment the counter
    pub post_body: String,
    pub post_loop: String,
}

pub trait Context: Debug {
    fn reset(&mut self);
    fn freeze(&mut self);
    fn is_frozen(&self) -> bool;
    fn throw_on_unknowns(&self) -> bool;
    fn finished_resolving_types(&mut self);
    fn set_parent(&mut self, parent: Rc<RefCell<dyn Context>>);
    fn get_parent(&self) -> Option<MutRc<dyn Context>>;
    fn global_scope(&self) -> MutRc<dyn Context>;

    fn get_cli_args(&self) -> Args;
    fn exec_mode(&self) -> ExecMode;
    fn std_asm_path(&self) -> &'static str;
    fn allow_overrides(&self) -> bool;

    fn set_current_dir_path(&mut self, path: &'static Path);
    fn get_current_dir_path(&self) -> &'static Path;

    fn get_id(&mut self) -> usize;
    fn get_anon_label(&mut self) -> String;
    fn get_global_anon_label(&mut self) -> String;

    fn declare(&mut self, symbol: SymbolDec, trace_interval: Interval) -> Result<SymbolDec, Error>;
    fn has_dec_with_id(&self, id: &str) -> bool;
    fn get_dec_from_id(&self, id: &str) -> SymbolDec;
    fn set_dec_as_defined(&mut self, id: &str, trace_interval: Interval) -> Result<(), Error>;
    fn update_dec_type(
        &mut self,
        id: &str,
        new_type: MutRc<dyn Type>,
        trace_interval: Interval,
    ) -> Result<(), Error>;
    fn get_new_local_var_offset(&self) -> usize;

    fn define(&mut self, symbol: SymbolDef, trace_interval: Interval) -> Result<(), Error>;
    fn define_global(&mut self, symbol: SymbolDef, trace_interval: Interval) -> Result<(), Error>;
    fn get_definitions(&self) -> (Vec<&SymbolDef>, Vec<&SymbolDef>);

    fn loop_labels_push(&mut self, lbl: LoopLabels);
    fn loop_labels_pop(&mut self) -> Option<LoopLabels>;
    fn loop_label_peak(&self) -> Option<LoopLabels>;

    fn stack_frame_push(&mut self, frame: CallStackFrame);
    fn stack_frame_pop(&mut self) -> Option<CallStackFrame>;
    fn stack_frame_peak(&self) -> Option<CallStackFrame>;

    #[allow(dead_code)]
    fn str(&self) -> String;

    fn concrete_type_cache_get(&self, id: String) -> Option<MutRc<dyn Type>>;
    fn concrete_type_cache_set(&mut self, id: String, t: MutRc<dyn Type>);
    fn clear_concrete_cache(&mut self);
    fn concrete_type_cache_remove(&mut self, id: &str);

    fn set_ignoring_definitions(&mut self, value: bool);
    fn is_ignoring_definitions(&self) -> bool;

    fn target(&self) -> Target;

    fn include_asm(&mut self, asm_path: String);
    fn get_included_asm_paths(&self) -> Vec<String>;
}
