use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;
use std::fmt::Debug;
use std::rc::Rc;

pub mod bin_op;
pub mod bool;
pub mod r#break;
pub mod r#continue;
pub mod empty_exec_root;
pub mod empty_global_var_decl;
pub mod empty_local_var_decl;
pub mod exec_root;
pub mod fn_call;
pub mod fn_declaration;
pub mod global_var_decl;
pub mod r#if;
pub mod int;
pub mod local_var_decl;
pub mod r#loop;
pub mod mutate_var;
pub mod pass;
pub mod r#return;
pub mod scope;
pub mod statements;
pub mod str;
pub mod class_declaration;
pub mod class_field_access;
pub mod class_init;
pub mod symbol_access;
pub mod type_expr;
pub mod types;
pub mod unary_op;

pub const ANON_PREFIX: &str = "_$_";
pub const STD_ASM: &str = include_str!("../../std/std.asm");

// (type of result of node, type of returned values from node and children)
pub type TypeCheckRes = (Rc<dyn Type>, Option<Rc<dyn Type>>);

pub trait Node: Debug {
    fn asm(&mut self, _ctx: MutRc<Context>) -> Result<String, Error> {
        Ok("".to_string())
    }
    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok((
            ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(),
            None,
        ))
    }
    fn pos(&mut self) -> Interval;
}
