use crate::ast::types::r#class::{ClassFieldType, ClassType};
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{unknown_symbol, Error};
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec};
use crate::util::MutRc;
use std::rc::Rc;

#[derive(Debug)]
pub struct ClassField {
    pub identifier: String,
    pub type_: MutRc<dyn Node>,
}

#[derive(Debug)]
pub struct ClassDeclarationNode {
    pub identifier: String,
    pub fields: Vec<ClassField>,
    pub position: Interval,
}

impl Node for ClassDeclarationNode {
    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }

        let fields = self
            .fields
            .iter()
            .map(|field| {
                let type_ = field.type_.borrow_mut().type_check(ctx.clone())?;
                Ok(ClassFieldType {
                    name: field.identifier.clone(),
                    type_: type_.0,
                })
            })
            .collect::<Result<Vec<ClassFieldType>, Error>>()?;

        let this_type = Rc::new(ClassType {
            name: self.identifier.clone(),
            fields,
        });

        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: self.identifier.clone(),
            is_constant: true,
            is_type: true,
            require_init: false,
            is_defined: true,
            is_param: false,
            type_: this_type.clone(),
        })?;

        Ok((this_type, None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
