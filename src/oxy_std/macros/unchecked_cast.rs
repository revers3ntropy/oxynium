use crate::ast::unchecked_type_cast_node::UncheckedTypeCastNode;
use crate::ast::AstNode;
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::oxy_std::macros::Macro;
use crate::position::Interval;
use crate::util::{new_mut_rc, MutRc};

pub struct UncheckedCastMacro {
    pub position: Interval,
    pub args: Vec<MutRc<dyn AstNode>>,
}

impl Macro for UncheckedCastMacro {
    fn resolve(&self, _ctx: MutRc<dyn Context>) -> Result<MutRc<dyn AstNode>, Error> {
        let mut args = self.args.clone();

        if args.len() != 2 {
            return Err(
                type_error(format!("macro `unchecked_cast` takes exactly 2 arguments"))
                    .set_interval(self.position.clone()),
            );
        }
        let new_type = args.remove(0);
        let value = args.remove(0);
        Ok(new_mut_rc(UncheckedTypeCastNode { value, new_type }))
    }
}
