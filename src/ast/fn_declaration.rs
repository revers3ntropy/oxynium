use crate::ast::class_declaration::method_id;
use crate::ast::{Node, TypeCheckRes};
use crate::context::CallStackFrame;
use crate::context::Context;
use crate::error::{invalid_symbol, syntax_error, type_error, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec, SymbolDef};
use crate::types::class::ClassType;
use crate::types::function::{FnParamType, FnType};
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug, Clone)]
pub struct Parameter {
    pub identifier: String,
    pub type_: Option<MutRc<dyn Node>>,
    pub default_value: Option<MutRc<dyn Node>>,
    pub position: Interval,
}

#[derive(Debug)]
pub struct FnDeclarationNode {
    pub identifier: Token,
    pub ret_type: MutRc<dyn Node>,
    pub params: Vec<Parameter>,
    pub body: Option<MutRc<dyn Node>>,
    pub params_scope: MutRc<Context>,
    pub is_external: bool,
    pub position: Interval,
    pub class: Option<MutRc<ClassType>>,
}

impl FnDeclarationNode {
    fn id(&self) -> String {
        match &self.class {
            Some(class) => method_id(
                class.borrow().name.clone(),
                self.identifier.literal.clone().unwrap(),
            ),
            None => self.identifier.literal.clone().unwrap(),
        }
    }
}

impl Node for FnDeclarationNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare function '{}' inside of another function.",
                self.identifier.str()
            ))
            .set_interval((self.pos().0, self.identifier.end.clone())));
        }

        if self.body.is_none() {
            return Ok("".to_string());
        }

        let end_label = ctx.borrow_mut().get_anon_label();
        ctx.borrow_mut().stack_frame_push(CallStackFrame {
            name: self.id(),
            params: self.params.iter().map(|a| a.identifier.clone()).collect(),
            ret_lbl: end_label.clone(),
        });

        let body = self
            .body
            .take()
            .unwrap()
            .borrow_mut()
            .asm(self.params_scope.clone())?;
        let params_scope = self.params_scope.clone();
        let params_scope = params_scope.borrow_mut();
        let (data_defs, text_defs) = params_scope.get_definitions();
        if text_defs.len() > 0 {
            return Err(type_error("Nested functions not allowed".to_string()));
        }

        let self_pos = self.pos();
        ctx.borrow_mut().define(
            SymbolDef {
                name: self.id(),
                data: None,
                text: Some(format!(
                    "
                    push rbp
                    mov rbp, rsp
                    times {} push 0
                    {body}
                {end_label}:
                    mov rsp, rbp
                    pop rbp
                    ret
                 ",
                    data_defs.len()
                )),
            },
            self_pos,
        )?;

        ctx.borrow_mut().stack_frame_pop();
        Ok("".to_string())
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
        self.params_scope.borrow_mut().set_parent(ctx.clone());
        self.params_scope.borrow_mut().allow_local_var_decls = true;

        // don't use param_scope so that the function can have params
        // with the same name as the function
        if !ctx.borrow_mut().allow_overrides {
            if ctx.borrow_mut().has_dec_with_id(self.id().as_str()) {
                return Err(type_error(format!(
                    "Symbol {} is already defined",
                    self.identifier.clone().literal.unwrap()
                ))
                .set_interval(self.position.clone()));
            }
        }
        let TypeCheckRes { t: ret_type, .. } =
            self.ret_type.borrow_mut().type_check(ctx.clone())?;

        let mut parameters: Vec<FnParamType> = Vec::new();

        let num_params = self.params.len();
        let mut seen_param_without_default = false;
        for i in 0..self.params.len() {
            let Parameter {
                identifier,
                type_,
                default_value,
                position,
            } = self.params[self.params.len() - i - 1].clone();

            if !can_declare_with_identifier(&identifier) {
                return Err(syntax_error("Invalid parameter name".to_string())
                    .set_interval(self.position.clone()));
            }

            let mut param_type = None;
            if let Some(type_) = type_ {
                param_type =
                    Some(type_.borrow_mut().type_check(ctx.clone())?.t);
            }
            if let Some(default_value) = default_value.clone() {
                if seen_param_without_default {
                    return Err(type_error(format!(
                        "Parameters after '{}' must have default values",
                        identifier
                    ))
                    .set_interval(self.position.clone()));
                }
                let default_value_type =
                    default_value.borrow_mut().type_check(ctx.clone())?.t;

                if param_type.is_none() {
                    param_type = Some(default_value_type.clone());
                }
                if !param_type
                    .clone()
                    .unwrap()
                    .borrow()
                    .contains(default_value_type.clone())
                {
                    return Err(type_error(format!(
                        "Default value for parameter '{}' is not of type {}",
                        identifier,
                        param_type.unwrap().borrow().str()
                    ))
                    .set_interval(default_value.borrow_mut().pos()));
                }
            } else {
                seen_param_without_default = true;
            }

            if param_type.is_none() {
                return Err(type_error(format!(
                    "Parameter '{}' must have a type",
                    identifier
                ))
                .set_interval(position));
            }

            parameters.insert(
                0,
                FnParamType {
                    name: identifier.clone(),
                    type_: param_type.clone().unwrap(),
                    default_value,
                    position,
                },
            );

            let self_pos = self.pos();
            self.params_scope.borrow_mut().declare(
                SymbolDec {
                    name: identifier.clone(),
                    id: format!(
                        "qword [rbp + {}]",
                        8 * ((num_params - (i + 1)) + 2)
                    ),
                    is_constant: true,
                    is_type: false,
                    require_init: false,
                    is_defined: true,
                    is_param: true,
                    type_: param_type.unwrap().clone(),
                    position: self.position.clone(),
                },
                self_pos,
            )?;
        }

        let this_type = new_mut_rc(FnType {
            name: self.id(),
            ret_type: ret_type.clone(),
            parameters,
        });
        // declare in the parent context
        ctx.borrow_mut().declare(
            SymbolDec {
                name: self.id(),
                id: self.id(),
                is_constant: true,
                is_type: false,
                require_init: !self.is_external,
                is_defined: self.body.is_some(),
                is_param: false,
                type_: this_type.clone(),
                position: self.pos(),
            },
            self.pos(),
        )?;

        if let Some(ref body) = self.body {
            let TypeCheckRes {
                t: body_ret_type,
                is_returned,
                ..
            } = body.borrow().type_check(self.params_scope.clone())?;
            let body_ret_type = if is_returned {
                body_ret_type
            } else {
                get_type!(ctx, "Void")
            };
            if !ret_type.borrow().contains(body_ret_type.clone()) {
                return Err(type_error(format!(
                    "Function `{}` has return type `{}` but found `{}`",
                    self.identifier.clone().literal.unwrap(),
                    ret_type.borrow().str(),
                    body_ret_type.borrow().str()
                ))
                .set_interval(self.identifier.interval()));
            }
        }

        Ok(TypeCheckRes::from(this_type))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
