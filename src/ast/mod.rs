use crate::ast::str::StrNode;
use crate::ast::symbol_access::SymbolAccess;
use crate::ast::type_expr::TypeNode;
use crate::ast::type_expr_generic::GenericTypeNode;
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};
use std::fmt::Debug;

pub mod bin_op;
pub mod bool;
pub mod r#break;
pub mod class_declaration;
pub mod class_field_access;
pub mod class_init;
pub mod r#continue;
pub mod empty_exec_root;
pub mod empty_global_const_decl;
pub mod empty_local_var_decl;
pub mod exec_root;
pub mod fn_call;
pub mod fn_declaration;
pub mod global_const_decl;
pub mod r#if;
pub mod int;
pub mod local_var_decl;
pub mod macro_call;
pub mod mutate_var;
pub mod pass;
pub mod raw_asm;
pub mod r#return;
pub mod scope;
pub mod statements;
pub mod str;
pub mod symbol_access;
pub mod type_expr;
pub mod type_expr_generic;
pub mod type_known;
pub mod unary_op;
pub mod r#while;

pub const ANON_PREFIX: &str = "_$_";
pub const STD_ASM: &str = include_str!("../../std/std.asm");
pub const STD_DATA_ASM: &str =
    include_str!("../../std/std-data.asm");

#[macro_export]
macro_rules! get_type {
    ($ctx:expr, $name:expr) => {
        if $ctx.borrow().has_dec_with_id($name) {
            $ctx.borrow_mut()
                .get_dec_from_id($name)
                .type_
                .clone()
        } else {
            new_mut_rc(UnknownType {})
        }
    };
}

// (type of result of node, type of returned values from node and children)
#[derive(Debug, Clone)]
pub struct TypeCheckRes {
    t: MutRc<dyn Type>,
    is_returned: bool,
    always_returns: bool,
    unknowns: usize,
}

impl TypeCheckRes {
    fn from(t: MutRc<dyn Type>, unknowns: usize) -> Self {
        Self {
            t,
            is_returned: false,
            always_returns: false,
            unknowns,
        }
    }

    fn returns(
        always: bool,
        t: MutRc<dyn Type>,
        unknowns: usize,
    ) -> Self {
        Self {
            t,
            is_returned: true,
            always_returns: always,
            unknowns,
        }
    }

    fn from_ctx(
        ctx: &MutRc<Context>,
        name: &str,
        mut unknowns: usize,
    ) -> Self {
        let t: MutRc<dyn Type>;
        if !ctx.borrow().has_dec_with_id(name) {
            unknowns += 1;
            t = new_mut_rc(UnknownType {});
        } else {
            t = get_type!(ctx, name);
        }

        if t.borrow().is_unknown() {
            unknowns += 1;
        }

        Self {
            t,
            is_returned: false,
            always_returns: false,
            unknowns,
        }
    }

    fn unknown() -> Self {
        Self {
            t: new_mut_rc(UnknownType {}),
            is_returned: false,
            always_returns: false,
            unknowns: 1,
        }
    }
    fn unknown_and(unknowns: usize) -> Self {
        Self {
            t: new_mut_rc(UnknownType {}),
            is_returned: false,
            always_returns: false,
            unknowns: unknowns + 1,
        }
    }
}

pub trait AstNode: Debug {
    fn setup(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<(), Error> {
        Ok(())
    }
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_ctx(&ctx, "Void", 0))
    }
    fn asm(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn pos(&self) -> Interval;

    fn as_str_node(&self) -> Option<StrNode> {
        None
    }
    fn as_type_expr(&self) -> Option<TypeNode> {
        None
    }
    fn as_type_generic_expr(
        &self,
    ) -> Option<GenericTypeNode> {
        None
    }
    fn as_symbol_access(&self) -> Option<SymbolAccess> {
        None
    }
}
