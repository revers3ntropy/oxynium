pub mod exec_root;
pub mod int;
pub mod bin_op;
pub mod unary_op;
pub mod fn_call;
pub mod statements;
pub mod str;
pub mod symbol_access;
pub mod global_var_decl;
pub mod mutate_var;
pub mod for_loop;
pub mod r#break;
pub mod r#if;
pub mod types;
pub mod type_expr;
pub mod fn_declaration;
pub mod r#continue;
pub mod pass;
pub mod scope;
pub mod r#return;
pub mod local_var_decl;

use std::fmt::Debug;
use std::rc::Rc;
use crate::ast::types::Type;
use crate::context::{Ctx};
use crate::error::Error;

pub const ANON_PREFIX: &str = "_$_";

// (type of result of node, type of returned values from node and children)
pub type TypeCheckRes = (Rc<Type>, Option<Rc<Type>>);

pub trait Node: Debug {
    fn asm(&mut self, _ctx: Ctx) -> Result<String, Error> {
        Ok("".to_string())
    }
    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), None))
    }
}

