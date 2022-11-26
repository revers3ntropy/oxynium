pub mod exec_root_node;
pub mod int_node;
pub mod bin_op_node;
pub mod arith_unary_op_node;
pub mod fn_call_node;
pub mod statements_node;
pub mod str_node;
pub mod symbol_access;
pub mod const_decl;
pub mod glob_var_decl;
pub mod mutate_var;
pub mod for_loop;
pub mod break_node;
pub mod if_node;

pub const ANON_PREFIX: &str = "__ANON_";

use std::fmt::Debug;
use crate::context::Context;
use crate::error::Error;

pub trait Node: Debug {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error>;
}