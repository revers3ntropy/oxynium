use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::types::Type;
use crate::util::MutRc;
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
pub mod mutate_var;
pub mod pass;
pub mod r#return;
pub mod scope;
pub mod statements;
pub mod str;
pub mod symbol_access;
pub mod type_expr;
pub mod unary_op;
pub mod r#while;

pub const ANON_PREFIX: &str = "_$_";
pub const STD_ASM: &str = include_str!("../../std/std.asm");
pub const STD_DATA_ASM: &str = include_str!("../../std/std-data.asm");

#[macro_export]
macro_rules! get_type {
    ($ctx:expr, $name:expr) => {
        $ctx.borrow_mut().get_dec_from_id($name).type_.clone()
    };
}

// (type of result of node, type of returned values from node and children)
pub struct TypeCheckRes {
    t: MutRc<dyn Type>,
    is_returned: bool,
    // unknowns_exist: bool,
    // progress_made: bool,
}

impl TypeCheckRes {
    fn from(t: MutRc<dyn Type>) -> Self {
        Self {
            t,
            is_returned: false,
            // unknowns_exist: false,
            // progress_made: false,
        }
    }

    fn from_return(t: MutRc<dyn Type>) -> Self {
        Self {
            t,
            is_returned: true,
            // unknowns_exist: false,
            // progress_made: false,
        }
    }

    fn from_ctx(ctx: &MutRc<Context>, name: &str) -> Self {
        Self {
            t: get_type!(ctx, name),
            is_returned: false,
            // unknowns_exist: false,
            // progress_made: false,
        }
    }
}

pub trait Node: Debug {
    fn asm(&mut self, _ctx: MutRc<Context>) -> Result<String, Error> {
        Ok("".to_string())
    }
    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_ctx(&ctx, "Void"))
    }
    fn pos(&self) -> Interval;
}
