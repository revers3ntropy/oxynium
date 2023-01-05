use crate::ast::fn_declaration::FnDeclarationNode;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{invalid_symbol, type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{
    can_declare_with_identifier, SymbolDec,
};
use crate::types::class::{ClassFieldType, ClassType};
use crate::types::function::FnType;
use crate::util::{new_mut_rc, MutRc};
use std::any::Any;
use std::collections::HashMap;

pub fn method_id(
    class_name: String,
    method_name: String,
) -> String {
    format!("{}.{}", class_name, method_name)
}

pub fn operator_method_id(
    class_name: String,
    operator: Token,
) -> String {
    format!(
        "{}._$_op_{}",
        class_name,
        operator.overload_op_id().unwrap()
    )
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
    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        let mut asm = "".to_string();
        for method in self.methods.iter() {
            let res =
                method.borrow_mut().asm(ctx.clone())?;
            asm.push_str(res.as_str());
        }

        Ok(asm)
    }
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        if !can_declare_with_identifier(
            &self.identifier.clone().literal.unwrap(),
        ) {
            return Err(invalid_symbol(
                self.identifier.clone().literal.unwrap(),
            )
            .set_interval(self.identifier.interval()));
        }

        let this_type: MutRc<ClassType>;
        if ctx.borrow().is_frozen() {
            let this_type_any = ctx
                .borrow()
                .get_dec_from_id(
                    &self
                        .identifier
                        .clone()
                        .literal
                        .unwrap(),
                )
                .type_;
            unsafe {
                this_type = (&*(&this_type_any
                    as *const dyn Any
                    as *const MutRc<ClassType>))
                    .clone();
            }
        } else {
            this_type = new_mut_rc(ClassType {
                name: self
                    .identifier
                    .clone()
                    .literal
                    .unwrap(),
                fields: HashMap::new(),
                methods: HashMap::new(),
                is_primitive: self.is_primitive,
            });
        }

        for field in self.fields.iter() {
            let type_ = field
                .type_
                .borrow_mut()
                .type_check(ctx.clone())?;
            unknowns += type_.unknowns;
            let mut stack_offset =
                this_type.borrow().fields.len() * 8;
            if this_type
                .borrow_mut()
                .fields
                .contains_key(&field.identifier.clone())
            {
                // preserve stack offset if already exists
                stack_offset = this_type
                    .borrow()
                    .field_offset(field.identifier.clone());
            }
            this_type.borrow_mut().fields.insert(
                field.identifier.clone(),
                ClassFieldType {
                    name: field.identifier.clone(),
                    type_: type_.t,
                    stack_offset,
                },
            );
        }

        let self_pos = self.pos();
        for method in self.methods.iter() {
            let mut method = method.borrow_mut();
            method.class = Some(this_type.clone());

            // This is where the context reference is handed down so the
            // method's context is attached to the global context tree
            let method_type_res =
                method.type_check(ctx.clone())?;
            unknowns += method_type_res.unknowns;

            if !method.is_external && method.body.is_none()
            {
                return Err(type_error(format!(
                    "Non-external method '{}' requires a body",
                    method.identifier.clone().literal.unwrap()
                ))
                .set_interval(self_pos));
            }

            unsafe {
                let fn_ = (&*(&method_type_res.t
                    as *const dyn Any
                    as *const Option<MutRc<FnType>>))
                    .clone()
                    .unwrap();
                this_type.borrow_mut().methods.insert(
                    fn_.borrow().name.clone(),
                    fn_.clone(),
                );
            }
        }

        if !ctx.borrow().is_frozen() {
            ctx.borrow_mut().declare(
                SymbolDec {
                    name: self
                        .identifier
                        .clone()
                        .literal
                        .unwrap(),
                    id: self
                        .identifier
                        .clone()
                        .literal
                        .unwrap(),
                    is_constant: true,
                    is_type: true,
                    require_init: false,
                    is_defined: true,
                    is_param: false,
                    type_: this_type.clone(),
                    position: self.pos(),
                },
                (
                    self.pos().0,
                    self.identifier.interval().1,
                ),
            )?;
        }

        Ok(TypeCheckRes::from(this_type, unknowns))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
