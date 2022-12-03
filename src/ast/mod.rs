pub mod exec_root;
pub mod int;
pub mod bin_op;
pub mod unary_op;
pub mod fn_call;
pub mod statements;
pub mod str;
pub mod symbol_access;
pub mod global_const;
pub mod mutate_var;
pub mod for_loop;
pub mod r#break;
pub mod r#if;
pub mod types;
pub mod type_expr;
pub mod fn_declaration;
pub mod type_wrapper;
pub mod r#continue;
pub mod pass;
pub mod context;

use std::fmt::Debug;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;

pub const ANON_PREFIX: &str = "_$_ANON_";

pub trait Node: Debug {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error>;
    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error>;
}

