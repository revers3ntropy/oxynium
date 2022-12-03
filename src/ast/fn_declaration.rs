use std::collections::HashMap;
use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Context, Symbol};
use crate::error::{Error, type_error_unstructured};

#[derive(Debug)]
pub struct FnDeclarationNode {
    pub identifier: String,
    pub ret_type: Box<dyn Node>,
    pub params: HashMap<String, Box<dyn Node>>,
    pub body: Option<Box<dyn Node>>
}

impl Node for FnDeclarationNode {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        if !ctx.allow_overrides && ctx.has_with_id(self.identifier.clone().as_str()) {
            return Err(type_error_unstructured(format!("Symbol {} is already defined", self.identifier)))
        }
        let ret_type = self.ret_type.type_check(ctx);

        let mut children = vec![ret_type?];
        for param in self.params.values_mut() {
            children.push(param.type_check(ctx)?);
        }

        let this_type = Type {
            id: 100,
            name: "Fn".to_owned(),
            children
        };
        if let Some(mut body_node) = self.body.take() {
            let body = body_node.asm(ctx)?;
            ctx.declare_glob_var(Symbol {
                name: self.identifier.clone(),
                data: None,
                text: Some(format!("
                        push rbp
                        mov rbp, rsp
                        {body}
                        mov rsp, rbp
                        pop rbp
                        ret
                     ")),
                constant: true,
                type_: Box::new(this_type.clone())
            });
        } else {
            ctx.declare(Symbol {
                name: self.identifier.clone(),
                data: None,
                text: Some("ret".to_string()),
                constant: true,
                type_: Box::new(this_type.clone())
            });
        }

        Ok(Box::new(this_type))
    }
}
