use crate::ast::pass::PassNode;
use crate::ast::scope::ScopeNode;
use crate::ast::str::StrNode;
use crate::ast::AstNode;
use crate::compile::generate_ast;
use crate::context::scope::Scope;
use crate::context::Context;
use crate::error::{type_error, Error, ErrorSource};
use crate::oxy_std::macros::Macro;
use crate::position::Interval;
use crate::target::Target;
use crate::util::{new_mut_rc, read_file};
use crate::util::{string_to_static_str, MutRc};
use std::path::Path;

pub struct IncludeMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn AstNode>>,
}

impl IncludeMacro {
    fn should_include(
        &self,
        ctx: MutRc<dyn Context>,
        target_node: Option<StrNode>,
    ) -> Result<bool, Error> {
        if target_node.is_none() {
            return Err(
                type_error(format!("Arguments to macro `asm` must be string literals"))
                    .set_interval(self.position.clone()),
            );
        }

        let target_node = target_node.unwrap();
        let target = target_node.value.clone().literal.unwrap();

        match ctx.borrow().target() {
            Target::X86_64Linux => Ok(target == Target::X86_64Linux.as_str()),
            Target::MACOS => Ok(target == Target::MACOS.as_str()),
        }
    }

    fn get_path(&self, ctx: MutRc<dyn Context>) -> Result<Option<String>, Error> {
        let args = self.args.clone();

        let path_node;
        match args.len() {
            1 => {
                path_node = args[0].borrow().as_str_node();
            }
            2 => {
                if !self.should_include(ctx.clone(), args[0].borrow().as_str_node())? {
                    return Ok(None);
                }
                path_node = args[1].borrow().as_str_node();
            }
            _ => {
                return Err(type_error(format!("macro `include` takes 1-2 argument"))
                    .set_interval(self.position.clone()));
            }
        }

        if path_node.is_none() {
            return Err(
                type_error(format!("Arguments to macro `asm` must be string literals"))
                    .set_interval(self.position.clone()),
            );
        }

        let path_node = path_node.unwrap();
        let path_str = path_node.value.clone().literal.unwrap();

        let path = ctx.borrow().get_current_dir_path();
        let path = path.join(path_str);
        Ok(Some(path.to_str().unwrap().to_string()))
    }
}

impl Macro for IncludeMacro {
    fn resolve(&self, ctx: MutRc<dyn Context>) -> Result<MutRc<dyn AstNode>, Error> {
        let path = self.get_path(ctx.clone())?;
        if path.is_none() {
            return Ok(new_mut_rc(PassNode {
                position: self.position.clone(),
            }));
        }
        let path = path.unwrap();

        let read_result = read_file(path.as_str())?;

        let err_source = ErrorSource {
            file_name: path.clone(),
            source: read_result.clone(),
        };

        let ast_res = generate_ast(&ctx.borrow().get_cli_args(), read_result, path.clone());

        if let Err(mut err) = ast_res {
            err.try_set_source(err_source);
            return Err(err);
        }
        let ast = ast_res.unwrap();

        let ctx = Scope::new_global(ctx.clone());

        let file_path = unsafe { Path::new(string_to_static_str(path)) };
        ctx.borrow_mut()
            .set_current_dir_path(file_path.parent().unwrap_or(file_path));

        return Ok(new_mut_rc(ScopeNode {
            position: self.position.clone(),
            body: ast,
            ctx: Some(ctx),
            err_source: Some(err_source),
        }));
    }
}
