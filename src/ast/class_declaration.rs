use crate::ast::fn_declaration::FnDeclarationNode;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{invalid_symbol, type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec};
use crate::types::class::{ClassFieldType, ClassType};
use crate::types::function::FnType;
use crate::types::Type;
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
    pub identifier: Token,
    pub fields: Vec<ClassField>,
    pub methods: Vec<MutRc<FnDeclarationNode>>,
    pub position: Interval,
    pub is_primitive: bool,
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
    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !can_declare_with_identifier(
            &self.identifier.clone().literal.unwrap(),
        ) {
            return Err(invalid_symbol(
                self.identifier.clone().literal.unwrap(),
            )
            .set_interval(self.identifier.interval()));
        }

        let this_type = new_mut_rc(ClassType {
            name: self.identifier.clone().literal.unwrap(),
            fields: vec![],
            methods: vec![],
            is_primitive: self.is_primitive,
        });

        ctx.borrow_mut().declare(
            SymbolDec {
                name: self.identifier.clone().literal.unwrap(),
                id: self.identifier.clone().literal.unwrap(),
                is_constant: true,
                is_type: true,
                require_init: false,
                is_defined: true,
                is_param: false,
                type_: this_type.clone(),
                position: self.pos(),
            },
            (self.pos().0, self.identifier.interval().1),
        )?;

        for field in self.fields.iter() {
            let type_ = field.type_.borrow_mut().type_check(ctx.clone())?;
            this_type.borrow_mut().fields.push(ClassFieldType {
                name: field.identifier.clone(),
                type_: type_.0,
            });
        }

        let self_pos = self.pos();
        for method in self.methods.iter() {
            let mut method = method.borrow_mut();
            method.class = Some(this_type.clone());

            // This is where the context reference is handed down so the
            // method's context is attached to the global context tree
            let (method_type, _) = method.type_check(ctx.clone())?;

            if !method.is_external && method.body.is_none() {
                return Err(type_error(format!(
                    "Non-external method '{}' requires a body",
                    method.identifier.clone().literal.unwrap()
                ))
                .set_interval(self_pos));
            }

            if method.params.len() < 1 {
                return Err(type_error(format!(
                    "Method '{}' must have 'self' parameter",
                    method.identifier.clone().literal.unwrap()
                ))
                .set_interval(method.pos()));
            }

            let method_first_param = method.params[0].type_.take();
            if method_first_param.is_none() {
                return Err(type_error(format!(
                    "Method '{}' must have 'self' parameter",
                    method.identifier.clone().literal.unwrap()
                ))
                .set_interval(method.pos()));
            }

            let (first_param_type, _) = method_first_param
                .clone()
                .unwrap()
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
            // give back after taking
            method.params[0].type_ = Some(method_first_param.unwrap());

            unsafe {
                let fn_ = &*(&method_type as *const dyn Any
                    as *const Option<MutRc<FnType>>);
                this_type.borrow_mut().methods.push(fn_.clone().unwrap());
            }
        }

        Ok((this_type, None))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
