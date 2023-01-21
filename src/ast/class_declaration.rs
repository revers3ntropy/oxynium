use crate::ast::fn_declaration::FnDeclarationNode;
use crate::ast::type_known::KnownTypeNode;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{invalid_symbol, type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{
    can_declare_with_identifier, SymbolDec,
};
use crate::types::class::{ClassFieldType, ClassType};
use crate::types::function::FnType;
use crate::types::generic::GenericType;
use crate::types::Type;
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
    pub type_: MutRc<dyn AstNode>,
}

#[derive(Debug)]
pub struct ClassDeclarationNode {
    pub identifier: Token,
    pub fields: Vec<ClassField>,
    pub methods: Vec<MutRc<FnDeclarationNode>>,
    pub position: Interval,
    pub is_primitive: bool,
    pub template_params: Vec<Token>,
    pub template_ctx: MutRc<Context>,
    pub is_exported: bool,
}

impl AstNode for ClassDeclarationNode {
    fn setup(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<(), Error> {
        if !can_declare_with_identifier(
            &self.identifier.clone().literal.unwrap(),
        ) {
            return Err(invalid_symbol(
                self.identifier.clone().literal.unwrap(),
            )
            .set_interval(self.identifier.interval()));
        }

        self.template_ctx
            .borrow_mut()
            .set_parent(ctx.clone());

        for field in &self.fields {
            field
                .type_
                .borrow_mut()
                .setup(self.template_ctx.clone())?;
        }
        for method in &self.methods {
            method
                .borrow_mut()
                .setup(self.template_ctx.clone())?;
        }
        Ok(())
    }

    fn type_check(
        &self,
        mut ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if self.is_exported {
            let parent = ctx.borrow().get_parent();
            if let Some(parent) = parent {
                ctx = parent;
            }
        }

        let mut unknowns = 0;

        for template in self.template_params.iter() {
            self.template_ctx.borrow_mut().declare(
                SymbolDec {
                    name: template.literal.clone().unwrap(),
                    id: template.literal.clone().unwrap(),
                    is_constant: true,
                    is_type: true,
                    type_: new_mut_rc(GenericType {
                        identifier: template.clone(),
                    }),
                    require_init: false,
                    is_defined: false,
                    is_param: false,
                    position: template.interval(),
                },
                template.interval(),
            )?;
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
            let mut generic_args = HashMap::new();
            for template in self.template_params.iter() {
                generic_args.insert(
                    template.literal.clone().unwrap(),
                    new_mut_rc(GenericType {
                        identifier: template.clone(),
                    })
                        as MutRc<dyn Type>,
                );
            }
            let generic_params_order = self
                .template_params
                .iter()
                .map(|t| t.literal.clone().unwrap())
                .collect();

            this_type = new_mut_rc(ClassType {
                name: self
                    .identifier
                    .clone()
                    .literal
                    .unwrap(),
                fields: HashMap::new(),
                methods: HashMap::new(),
                is_primitive: self.is_primitive,
                id: ctx.borrow_mut().get_id(),
                generic_args,
                generic_params_order,
            });
        }

        for field in self.fields.iter() {
            let type_ = field
                .type_
                .borrow()
                .type_check(self.template_ctx.clone())?;
            unknowns += type_.unknowns;
            let mut stack_offset =
                this_type.borrow().fields.len() * 8;
            if this_type
                .borrow()
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
            let has_self_arg =
                method.params.first().is_some()
                    && method
                        .params
                        .first()
                        .unwrap()
                        .identifier
                        == "self";
            if has_self_arg {
                let first_param =
                    method.params.first().unwrap().clone();
                method.params.remove(0);

                let mut self_param = first_param.clone();
                self_param.type_ =
                    Some(new_mut_rc(KnownTypeNode {
                        t: this_type.clone(),
                        pos: first_param.position.clone(),
                    }));
                method.params.insert(0, self_param);
            }

            // This is where the context reference is handed down so the
            // method's context is attached to the global context tree
            let method_type_res = method
                .type_check(self.template_ctx.clone())?;
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

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
