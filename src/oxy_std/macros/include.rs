use crate::ast::scope::ScopeNode;
use crate::ast::AstNode;
use crate::compile::generate_ast;
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::oxy_std::macros::Macro;
use crate::position::Interval;
use crate::util::MutRc;
use crate::util::{new_mut_rc, read_file};

pub struct IncludeMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn AstNode>>,
}

impl Macro for IncludeMacro {
    fn resolve(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<MutRc<dyn AstNode>, Error> {
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
        let path = path_node.value.clone().literal.unwrap();

        let read_result = read_file(path.as_str())?;
        let ast =
            generate_ast(ctx.clone(), read_result, path)?;

        return Ok(new_mut_rc(ScopeNode {
            position: self.position.clone(),
            body: ast,
            ctx: Context::new(
                ctx.borrow().cli_args.clone(),
            ),
        }));
    }
}
