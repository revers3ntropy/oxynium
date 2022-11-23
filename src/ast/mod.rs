pub mod exec_root_node;
pub mod int_node;
pub mod arith_bin_op_node;
pub mod term_bin_op_node;
pub mod arith_unary_op_node;
pub mod fn_call_node;
pub mod statements_node;
pub mod str_node;
pub mod symbol_access;
pub mod const_decl;

pub const ANON_DATA_PREFIX: &str = "__ANON_DATA_";

use std::fmt::Debug;
use crate::context::Context;
use crate::error::Error;

pub trait Node: Debug {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error>;
}