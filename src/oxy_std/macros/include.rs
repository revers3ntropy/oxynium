use crate::ast::scope::ScopeNode;
use crate::ast::AstNode;
use crate::compile::generate_ast;
use crate::context::scope::Scope;
use crate::context::Context;
use crate::error::{type_error, Error, ErrorSource};
use crate::oxy_std::macros::Macro;
use crate::position::Interval;
use crate::util::{new_mut_rc, read_file};
use crate::util::{string_to_static_str, MutRc};
use std::path::Path;

pub struct IncludeMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn AstNode>>,
}

impl IncludeMacro {
    fn get_path(
        &self,
        ctx: MutRc<dyn Context>,
    ) -> Result<String, Error> {
        let args = self.args.clone();

        if args.len() != 1 {
            return Err(type_error(format!(
                "macro `include` takes exactly 1 argument"
            ))
            .set_interval(self.position.clone()));
        }

        let path_node = args[0].borrow().as_str_node();

        if path_node.is_none() {
            return Err(type_error(format!(
                "First argument to macro `asm` must be a string literal"
            )).set_interval(self.position.clone()));
        }

        let path_node = path_node.unwrap();
        let path_str =
            path_node.value.clone().literal.unwrap();

        let path = ctx.borrow().get_current_dir_path();
        let path = path.join(path_str);
        Ok(path.to_str().unwrap().to_string())
    }
}

impl Macro for IncludeMacro {
    fn resolve(
        &self,
        ctx: MutRc<dyn Context>,
    ) -> Result<MutRc<dyn AstNode>, Error> {
        let path = self.get_path(ctx.clone())?;

        let read_result = read_file(path.as_str())?;

        let err_source = ErrorSource {
            file_name: path.clone(),
            source: read_result.clone(),
        };

        let ast_res = generate_ast(
            &ctx.borrow().get_cli_args(),
            read_result,
            path.clone(),
        );

        if let Err(mut err) = ast_res {
            err.try_set_source(err_source);
            return Err(err);
        }
        let ast = ast_res.unwrap();

        let ctx = Scope::new_global(ctx.clone());

        let file_path = unsafe {
            Path::new(string_to_static_str(path))
        };
        ctx.borrow_mut().set_current_dir_path(
            file_path.clone().parent().unwrap_or(file_path),
        );

        return Ok(new_mut_rc(ScopeNode {
            position: self.position.clone(),
            body: ast,
            ctx: Some(ctx),
            err_source: Some(err_source),
        }));
    }
}
