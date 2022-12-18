use crate::ast::fn_declaration::FnDeclarationNode;
use crate::ast::types::function::FnType;
use crate::ast::types::r#class::{ClassFieldType, ClassType};
use crate::ast::types::Type;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec};
use crate::util::{new_mut_rc, MutRc};
use std::any::Any;

pub fn method_id(class_name: String, method_name: String) -> String {
    format!("{}.{}", class_name, method_name)
}

#[derive(Debug)]
pub struct ClassField {
    pub identifier: String,
    pub type_: MutRc<dyn Node>,
}

#[derive(Debug)]
pub struct ClassDeclarationNode {
    pub identifier: String,
    pub fields: Vec<ClassField>,
    pub methods: Vec<MutRc<FnDeclarationNode>>,
    pub position: Interval,
}

impl Node for ClassDeclarationNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = "".to_string();
        for method in self.methods.iter() {
            let res = method.borrow_mut().asm(ctx.clone())?;
            asm.push_str(res.as_str());
        }

        Ok(asm)
    }
    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }

        let this_type = new_mut_rc(ClassType {
            name: self.identifier.clone(),
            fields: vec![],
            methods: vec![],
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

        for field in self.fields.iter() {
            let type_ = field.type_.borrow_mut().type_check(ctx.clone())?;
            this_type.borrow_mut().fields.push(ClassFieldType {
                name: field.identifier.clone(),
                type_: type_.0,
            });
        }

        for method in self.methods.iter() {
            let mut method = method.borrow_mut();
            method.class = Some(this_type.clone());

            // This is where the context reference is handed down so the
            // method's context is attached to the global context tree
            let (method_type, _) = method.type_check(ctx.clone())?;

            if method.params.len() < 1 {
                return Err(type_error(format!(
                    "Method '{}' must have 'self' parameter",
                    method.identifier.clone().literal.unwrap()
                ))
                .set_interval(method.pos()));
            }

            let (first_param_type, _) = method.params[0]
                .type_
                .borrow_mut()
                .type_check(ctx.clone())?;
            if !this_type.borrow().contains(first_param_type) {
                return Err(type_error(format!(
                    "Method '{}' must have 'self' parameter of type '{}'",
                    method.identifier.clone().literal.unwrap(),
                    this_type.borrow().str()
                ))
                .set_interval(method.pos()));
            }

            unsafe {
                let fn_ = &*(&method_type as *const dyn Any
                    as *const Option<MutRc<FnType>>);
                this_type.borrow_mut().methods.push(fn_.clone().unwrap());
            }
        }

        Ok((this_type, None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
