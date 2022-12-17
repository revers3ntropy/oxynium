use crate::ast::class_declaration::method_id;
use crate::ast::types::Type;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ClassMethodCallNode {
    pub base: MutRc<dyn Node>,
    pub name: Token,
    pub args: Vec<MutRc<dyn Node>>,
    pub position: Interval,
    pub use_return_value: bool,
}

impl Node for ClassMethodCallNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = format!("");

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        let (base_type_any, _) =
            self.base.borrow_mut().type_check(ctx.clone())?;
        let base_type = base_type_any.borrow_mut().as_class().unwrap();

        asm.push_str(&format!(
            "
            call {}
            {}
            {}
        ",
            method_id(
                base_type.name.clone(),
                self.name.clone().literal.unwrap()
            ),
            if self.args.len() > 0 {
                format!("times {} pop rcx", self.args.len())
            } else {
                "".to_string()
            },
            if self.use_return_value {
                "push rax"
            } else {
                ""
            }
        ));

        Ok(asm)
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let (base_type_any, _) =
            self.base.borrow_mut().type_check(ctx.clone())?;
        let base_type = base_type_any.borrow().as_class();
        if base_type.is_none() {
            return Err(type_error(format!(
                "Cannot access method of non-class type '{}'",
                base_type_any.borrow_mut().str()
            ))
            .set_interval(self.position.clone()));
        }
        let base_type = base_type.unwrap();

        let method_type =
            base_type.method_type(&self.name.clone().literal.unwrap());
        if method_type.is_none() {
            return Err(type_error(format!(
                "Class '{}' does not have method '{}'",
                base_type.str(),
                self.name.clone().literal.unwrap(),
            ))
            .set_interval(self.position.clone()));
        }
        let method_type = method_type.unwrap();

        self.use_return_value = !method_type
            .borrow_mut()
            .ret_type
            .borrow()
            .contains(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone());

        Ok((method_type, None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
