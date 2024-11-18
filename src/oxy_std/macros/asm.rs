use crate::ast::raw_asm::RawAsmNode;
use crate::ast::AstNode;
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::oxy_std::macros::Macro;
use crate::position::Interval;
use crate::util::mut_rc;
use crate::util::MutRc;

pub struct AsmMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn AstNode>>,
}

impl Macro for AsmMacro {
    fn resolve(&self, _ctx: MutRc<dyn Context>) -> Result<MutRc<dyn AstNode>, Error> {
        let mut args = self.args.clone();

        if args.len() != 2 {
            return Err(type_error(format!("macro `asm` takes exactly 2 arguments"))
                .set_interval(self.position.clone()));
        }
        let type_arg = args.remove(0);
        let asm_arg = args.remove(0);
        if let Some(as_str_node) = asm_arg.borrow().as_str_node() {
            return Ok(mut_rc(RawAsmNode {
                asm: as_str_node.value.clone(),
                return_type: type_arg,
            }));
        }
        Err(
            type_error(format!("Argument to macro `asm` must be a string literal"))
                .set_interval(self.position.clone()),
        )
    }
}
