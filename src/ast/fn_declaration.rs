use crate::ast::types::function::{FnParamType, FnType};
use crate::ast::{Node, TypeCheckRes};
use crate::ast::class_declaration::method_id;
use crate::ast::types::class::ClassType;
use crate::context::CallStackFrame;
use crate::context::Context;
use crate::error::{invalid_symbol, syntax_error, type_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec, SymbolDef};
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug, Clone)]
pub struct Parameter {
    pub identifier: String,
    pub type_: MutRc<dyn Node>,
    pub default_value: Option<MutRc<dyn Node>>,
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
    pub class: Option<MutRc<ClassType>>
}

impl FnDeclarationNode {
    fn id(&self) -> String {
        match &self.class {
            Some(class) => method_id(
                class.borrow().name.clone(),
                self.identifier.literal.clone().unwrap()
            ),
            None => self.identifier.literal.clone().unwrap()
        }
    }
}

impl Node for FnDeclarationNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare function '{}' inside of another function.",
                self.identifier.str()
            )));
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
        let params_scope = self.params_scope.borrow_mut();
        let (data_defs, text_defs) = params_scope.get_definitions();
        if text_defs.len() > 0 {
            return Err(type_error("Nested functions not allowed".to_string()));
        }
        ctx.borrow_mut().define(SymbolDef {
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
        })?;

        ctx.borrow_mut().stack_frame_pop();
        Ok("".to_string())
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
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
                )).set_interval(self.position.clone()));
            }
        }
        let (ret_type, _) =
            self.ret_type.borrow_mut().type_check(ctx.clone())?;

        let mut parameters: Vec<FnParamType> = Vec::new();

        let num_params = self.params.len();
        let mut seen_param_without_default = false;
        for i in 0..self.params.len() {
            let Parameter {
                identifier,
                type_,
                default_value,
            } = self.params[self.params.len() - i - 1].clone();

            if !can_declare_with_identifier(&identifier) {
                return Err(syntax_error("Invalid parameter name".to_string())
                    .set_interval(self.position.clone()));
            }

            let param_type = type_.borrow_mut().type_check(ctx.clone())?.0;
            if let Some(default_value) = default_value.clone() {
                if seen_param_without_default {
                    return Err(type_error(format!(
                        "Parameters after '{}' must have default values",
                        identifier
                    ))
                    .set_interval(self.position.clone()));
                }
                let default_value_type =
                    default_value.borrow_mut().type_check(ctx.clone())?.0;
                if !param_type.borrow().contains(default_value_type.clone()) {
                    return Err(type_error(format!(
                        "Default value for parameter '{}' is not of type {}",
                        identifier,
                        param_type.borrow().str()
                    ))
                    .set_interval(self.position.clone()));
                }
            } else {
                seen_param_without_default = true;
            }

            parameters.insert(
                0,
                FnParamType {
                    name: identifier.clone(),
                    type_: param_type.clone(),
                    default_value,
                },
            );

            self.params_scope.borrow_mut().declare(SymbolDec {
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
                type_: param_type.clone(),
            })?;
        }

        let this_type = new_mut_rc(FnType {
            name: self.id(),
            ret_type: ret_type.clone(),
            parameters,
        });
        // declare in the parent context
        ctx.borrow_mut().declare(SymbolDec {
            name: self.id(),
            id: self.id(),
            is_constant: true,
            is_type: false,
            require_init: !self.is_external,
            is_defined: self.body.is_some(),
            is_param: false,
            type_: this_type.clone(),
        })?;

        if let Some(body) = self.body.take() {
            let (_, body_ret_type) =
                body.borrow_mut().type_check(self.params_scope.clone())?;
            let body_ret_type = body_ret_type
                .unwrap_or(ctx.borrow_mut().get_dec_from_id("Void").unwrap().type_.clone());
            if !ret_type.borrow().contains(body_ret_type.clone()) {
                return Err(type_error(format!(
                    "Function {} has return type {} but found {}",
                    self.identifier.clone().literal.unwrap(),
                    ret_type.borrow().str(),
                    body_ret_type.borrow().str()
                ))
                .set_interval(self.position.clone()));
            }
            self.body = Some(body);
        }

        Ok((this_type, None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
