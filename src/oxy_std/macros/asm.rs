use crate::ast::raw_asm::RawAsmNode;
use crate::ast::Node;
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::oxy_std::macros::Macro;
use crate::position::Interval;
use crate::util::new_mut_rc;
use crate::util::MutRc;

pub struct AsmMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn Node>>,
}

impl Macro for AsmMacro {
    fn resolve(
        &self,
        _ctx: MutRc<Context>,
    ) -> Result<MutRc<dyn Node>, Error> {
        if self.args.len() != 1 {
            return Err(type_error(format!(
                "macro `asm` takes exactly one argument"
            ))
            .set_interval(self.position.clone()));
        }

        if let Some(as_str_node) =
            self.args[0].borrow().as_str_node()
        {
            return Ok(new_mut_rc(RawAsmNode {
                asm: as_str_node.value.clone(),
            }));
        }

        return Ok(self.args[0].clone());
    }
}
