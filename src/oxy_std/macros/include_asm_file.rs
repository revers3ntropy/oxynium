use crate::ast::include_asm_file::IncludeAsmFileNode;
use crate::ast::AstNode;
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::oxy_std::macros::Macro;
use crate::position::Interval;
use crate::util::new_mut_rc;
use crate::util::MutRc;

pub struct IncludeAsmFileMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn AstNode>>,
}

impl Macro for IncludeAsmFileMacro {
    fn resolve(&self, _ctx: MutRc<dyn Context>) -> Result<MutRc<dyn AstNode>, Error> {
        let mut args = self.args.clone();

        if args.len() != 1 {
            return Err(
                type_error(format!("macro `include_asm_file` takes exactly 1 argument"))
                    .set_interval(self.position.clone()),
            );
        }
        let arg = args.remove(0);
        if let Some(as_str_node) = arg.borrow().as_str_node() {
            return Ok(new_mut_rc(IncludeAsmFileNode {
                file_path: as_str_node.value.clone(),
            }));
        }
        Err(
            type_error(format!("Argument to macro `asm` must be a string literal"))
                .set_interval(self.position.clone()),
        )
    }
}
