use crate::ast::fn_declaration::FnDeclarationNode;
use crate::ast::types::function::{FnParamType, FnType};
use crate::ast::types::r#class::{ClassFieldType, ClassType};
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{unknown_symbol, Error};
use crate::position::Interval;
use crate::symbols::{can_declare_with_identifier, SymbolDec};
use crate::util::{MutRc, new_mut_rc};

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
        for method in self.methods.iter() {
            let id = method_id(
                self.identifier.clone(),
                method.borrow().identifier.clone(),
            );
            method.borrow_mut().identifier = id.clone();
            method.borrow_mut().asm(ctx.clone())?;
        }

        Ok("".to_string())
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
            methods: vec![]
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
            let old_id = method.borrow().identifier.clone();
            let mut method = method.borrow_mut();
            // This is where the context reference is handed down so the
            // method's context is attached to the global context tree
            let (ret_type, _) = method.type_check(ctx.clone())?;
            let params = method
                .params
                .iter()
                .map(|param| {
                    let type_ =
                        param.type_.borrow_mut().type_check(ctx.clone())?;
                    Ok(FnParamType {
                        name: param.identifier.clone(),
                        type_: type_.0,
                        default_value: param.default_value.clone(),
                    })
                })
                .collect::<Result<Vec<FnParamType>, Error>>()?;

            // doesn't matter as type checking doesn't happen on this declaration,
            // but otherwise the context complains tht we are defining an undeclared
            // symbol
            let void =
                ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone();
            let id = method_id(
                self.identifier.clone(),
                method.identifier.clone(),
            );
            ctx.borrow_mut().declare(SymbolDec {
                name: id.clone(),
                id,
                is_constant: true,
                is_type: false,
                type_: void,
                require_init: false,
                is_defined: false,
                is_param: false,
            })?;

            this_type.borrow_mut().methods.push(new_mut_rc(FnType {
                name: old_id,
                parameters: params,
                ret_type,
            }))
        }

        Ok((this_type, None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
