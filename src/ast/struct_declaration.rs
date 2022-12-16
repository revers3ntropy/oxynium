use crate::ast::types::r#struct::{StructFieldType, StructType};
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{unknown_symbol, Error};
use crate::position::Interval;
use crate::symbols::{is_valid_identifier, SymbolDec};
use crate::util::MutRc;
use std::rc::Rc;

#[derive(Debug)]
pub struct StructField {
    pub identifier: String,
    pub type_: MutRc<dyn Node>,
}

#[derive(Debug)]
pub struct StructDeclarationNode {
    pub identifier: String,
    pub fields: Vec<StructField>,
    pub position: Interval,
}

impl Node for StructDeclarationNode {
    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }

        let fields = self
            .fields
            .iter()
            .map(|field| {
                let type_ = field.type_.borrow_mut().type_check(ctx.clone())?;
                Ok(StructFieldType {
                    name: field.identifier.clone(),
                    type_: type_.0,
                })
            })
            .collect::<Result<Vec<StructFieldType>, Error>>()?;

        let this_type = Rc::new(StructType {
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
