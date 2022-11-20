pub mod exec_root_node;
pub mod int_node;
pub mod arith_bin_op_node;
pub mod term_bin_op_node;

pub(crate) const ANON_DATA_PREFIX: &str = "__anon_data_";

use std::fmt::Debug;
use crate::context::Context;

pub(crate) trait Node: Debug {
    fn asm(&mut self, ctx: &mut Context) -> String;
}