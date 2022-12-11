use std::process::id;
use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
use crate::ast::types::function::{FnParamType, FnType};
use crate::context::CallStackFrame;
use crate::error::{Error, syntax_error, type_error, unknown_symbol};
use crate::symbols::{is_valid_identifier, SymbolDec, SymbolDef};
use crate::context::Context;
use crate::util::MutRc;


#[derive(Debug)]
pub struct Parameter {
    pub identifier: String,
    pub type_: MutRc<dyn Node>,
    pub default_value: Option<MutRc<dyn Node>>,
}

#[derive(Debug)]
pub struct FnDeclarationNode {
    pub identifier: String,
    pub ret_type: MutRc<dyn Node>,
    pub params: Vec<Parameter>,
    pub body: Option<MutRc<dyn Node>>,
    pub params_scope: MutRc<Context>,
    pub is_external: bool,
}

impl Node for FnDeclarationNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare function '{}' inside of another function.",
                self.identifier
            )));
        }

        if self.body.is_none() {
            return Ok("".to_string());
        }

        let end_label = ctx.borrow_mut().get_anon_label();
        ctx.borrow_mut().stack_frame_push(CallStackFrame {
            name: self.identifier.clone(),
            params: self.params
                .iter()
                .map(|a| a.identifier.clone())
                .collect(),
            ret_lbl: end_label.clone()
        });

        let body = self.body.take().unwrap().borrow_mut()
            .asm(self.params_scope.clone())?;
        let params_scope = self.params_scope.borrow_mut();
        let (data_defs, text_defs) = params_scope.get_definitions();
        if text_defs.len() > 0 {
            return Err(type_error("Nested functions not allowed".to_string()));
        }
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            is_local: false,
            data: None,
            text: Some(format!("
                    push rbp
                    mov rbp, rsp
                    times {} push 0
                    {body}
                {end_label}:
                    mov rsp, rbp
                    pop rbp
                    ret
                 ", data_defs.len()))
        }, false)?;

        ctx.borrow_mut().stack_frame_pop();
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }
        self.params_scope.borrow_mut().set_parent(ctx.clone());
        self.params_scope.borrow_mut().allow_local_var_decls = true;

        // don't use param_scope so that the function can have params
        // with the same name as the function
        if !ctx.borrow_mut().allow_overrides {
            if ctx.borrow_mut().has_dec_with_id(self.identifier.clone().as_str()) {
                return Err(type_error(format!("Symbol {} is already defined",
                                              self.identifier)))
            }
        }
        let (ret_type, _) = self.ret_type.borrow_mut().type_check(ctx.clone())?;

        let mut parameters: Vec<FnParamType> = Vec::new();

        let num_params = self.params.len();
        let mut seen_param_without_default = false;
        for i in 0..self.params.len() {

            let Parameter { identifier, type_, default_value} =
                self.params.pop().unwrap();

            let param_type = type_.borrow_mut().type_check(ctx.clone())?.0;
            if let Some(default_value) = default_value.clone() {
                if seen_param_without_default {
                    return Err(type_error(format!(
                        "Parameters after '{}' must have default values",
                        identifier
                    )));
                }
                let default_value_type = default_value.borrow_mut().type_check(ctx.clone())?.0;
                if !param_type.contains(default_value_type.clone()) {
                    return Err(type_error(format!("Default value for parameter {} is not of type {}",
                                                  identifier, param_type.str())));
                }
            } else {
                seen_param_without_default = true;
            }

            parameters.push(FnParamType {
                name: identifier.clone(),
                type_: param_type.clone(),
                default_value
            });

            self.params_scope.borrow_mut().declare(SymbolDec {
                name: identifier.clone(),
                id: format!("qword [rbp+{}]", 8 * ((num_params - (i + 1)) + 2)),
                is_constant: true,
                is_type: false,
                require_init: false,
                is_defined: true,
                type_: param_type.clone()
            })?;
        }

        parameters.reverse();

        let this_type = Rc::new(FnType {
            name: self.identifier.clone(),
            ret_type: ret_type.clone(),
            parameters,
        });
        // declare in the parent context
        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: self.identifier.clone(),
            is_constant: true,
            is_type: false,
            require_init: !self.is_external,
            is_defined: self.body.is_some(),
            type_: this_type.clone()
        })?;

        if let Some(body) = self.body.take() {
            let (body_ret_type, _) = body.borrow_mut().type_check(self.params_scope.clone())?;
            if !ret_type.contains(body_ret_type.clone()) {
                return Err(type_error(format!(
                    "Function {} has return type {} but found {}",
                    self.identifier, ret_type.str(), body_ret_type.str()
                )));
            }
            self.body = Some(body);
        }

        Ok((this_type, None))
    }
}
