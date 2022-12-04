use std::rc::Rc;
use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{CallStackFrame, Ctx, SymbolDec, SymbolDef};
use crate::error::{Error, type_error};

pub type Params = Vec<Parameter>;

#[derive(Debug)]
pub struct Parameter {
    pub identifier: String,
    pub type_: Box<dyn Node>,
}

#[derive(Debug)]
pub struct FnDeclarationNode {
    pub identifier: String,
    pub ret_type: Box<dyn Node>,
    pub params: Vec<Parameter>,
    pub body: Option<Box<dyn Node>>,
    pub params_scope: Ctx
}

impl Node for FnDeclarationNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        if self.body.is_none() {
            return Ok("".to_string());
        }
        ctx.borrow_mut().stack_frame_push(CallStackFrame {
            name: self.identifier.clone()
        });

        let body = self.body.take().unwrap().asm(Rc::clone(&self.params_scope))?;
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            is_local: false,
            data: None,
            text: Some(format!("
                    push rbp
                    mov rbp, rsp
                    {body}
                _$_{}_end:
                    mov rsp, rbp
                    pop rbp
                    ret
                 ", self.identifier.clone()))
        }, false)?;
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        self.params_scope.borrow_mut().set_parent(Rc::clone(&ctx));

        // don't use param_scope so that the function can have params
        // with the same name as the function
        if !ctx.borrow_mut().allow_overrides {
            if ctx.borrow_mut().has_dec_with_id(self.identifier.clone().as_str()) {
                return Err(type_error(format!("Symbol {} is already defined",
                                              self.identifier)))
            }
        }
        let ret_type = self.ret_type.type_check(Rc::clone(&ctx));

        let mut children = vec![ret_type?];
        let num_params = self.params.len();
        for i in 0..self.params.len() {
            let Parameter { identifier, mut type_} =
                self.params.pop().unwrap();
            children.push(type_.type_check(Rc::clone(&ctx))?);

            self.params_scope.borrow_mut().declare(SymbolDec {
                name: identifier.clone(),
                id: format!("qword [rbp+{}]", 8 * ((num_params - (i + 1)) + 2)),
                is_constant: true,
                is_type: false,
                type_: children.last().unwrap().clone()
            })?;
        }

        let this_type = Type {
            id: ctx.borrow_mut().get_type_id(),
            name: "Fn".to_owned(),
            children
        };
        // declare in the parent context
        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id: self.identifier.clone(),
            is_constant: true,
            is_type: false,
            type_: Box::new(this_type.clone())
        })?;

        Ok(Box::new(this_type))
    }
}
