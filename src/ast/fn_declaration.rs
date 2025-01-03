use crate::ast::class_declaration::{method_id, operator_method_id};
use crate::ast::pass::PassNode;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::scope::Scope;
use crate::context::CallStackFrame;
use crate::context::Context;
use crate::error::{invalid_symbol, syntax_error, type_error, unknown_symbol, Error};
use crate::get_type;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::post_process::optimise::o1_enabled;
use crate::symbols::{can_declare_with_identifier, SymbolDec, SymbolDef};
use crate::types::function::{FnParamType, FnType};
use crate::types::generic::GenericType;
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub identifier: String,
    pub type_: Option<MutRc<dyn AstNode>>,
    pub default_value: Option<MutRc<dyn AstNode>>,
    pub position: Interval,
}

pub struct FnDeclarationNode {
    pub identifier: Token,
    pub ret_type: MutRc<dyn AstNode>,
    pub params: Vec<Parameter>,
    pub body: Option<MutRc<dyn AstNode>>,
    pub generic_parameters: Vec<Token>,
    pub params_scope: Option<MutRc<dyn Context>>,
    pub is_external: bool,
    pub position: Interval,
    pub class_name: Option<String>,
    pub has_usage: bool,
    pub is_exported: bool,
    pub is_anon: bool,
    pub should_infer_return_type: bool,
}

impl FnDeclarationNode {
    fn id(&self) -> String {
        if self.identifier.token_type == TokenType::Identifier {
            let self_id = self.identifier.literal.clone().unwrap();
            match &self.class_name {
                Some(class) => method_id(class.clone(), self_id),
                None => self_id,
            }
        } else {
            operator_method_id(self.class_name.clone().unwrap(), self.identifier.clone())
        }
    }

    fn get_generic_param_names(
        &self,
        ctx: MutRc<dyn Context>,
        //      (list of tokens which are the generic parameters, unknowns)
    ) -> Result<(Vec<Token>, usize), Error> {
        let mut generic_params = self.generic_parameters.clone();

        if self.params.len() < 1 || self.params.first().unwrap().identifier != "self" {
            // static methods don't get access to the
            // class's generic parameters
            return Ok((generic_params, 0));
        }

        if let Some(ref class_name) = self.class_name {
            if !ctx.borrow().has_dec_with_id(class_name) {
                if ctx.borrow().throw_on_unknowns() {
                    return Err(unknown_symbol(class_name.clone()));
                }
                return Ok((generic_params, 1));
            }

            let mut class = ctx.borrow().get_dec_from_id(class_name).type_;

            if let Some(type_type) = class.clone().borrow().as_type_type() {
                if let Some(class_type) = type_type.as_class() {
                    class = mut_rc(class_type)
                }
            }

            if let Some(class) = class.clone().borrow().as_class() {
                for generic in &class.generic_params_order {
                    if generic_params
                        .iter()
                        .filter(|t| t.literal == generic.literal)
                        .count()
                        > 0
                    {
                        return Err(type_error(format!(
                            "generic parameter `{}` is already defined in class `{}`",
                            generic.literal.clone().unwrap(),
                            class_name
                        ))
                        .set_interval(generic.interval()));
                    }
                    generic_params.push(generic.clone());
                }
            }
        }

        Ok((generic_params, 0))
    }

    fn declare_generic_parameters_on_own_scope(
        &self,
        ctx: MutRc<dyn Context>,
    ) -> Result<usize, Error> {
        let (generic_params, unknowns) = self.get_generic_param_names(ctx.clone())?;
        for generic_param in generic_params.iter() {
            if self
                .params_scope
                .clone()
                .unwrap()
                .borrow()
                .has_dec_with_id(&generic_param.literal.clone().unwrap())
            {
                continue;
            }
            self.params_scope.clone().unwrap().borrow_mut().declare(
                SymbolDec {
                    name: generic_param.literal.clone().unwrap(),
                    id: generic_param.literal.clone().unwrap(),
                    is_constant: true,
                    is_type: true,
                    is_func: false,
                    type_: mut_rc(GenericType {
                        identifier: generic_param.clone(),
                    }),
                    require_init: false,
                    is_defined: false,
                    is_param: false,
                    position: generic_param.interval(),
                },
                generic_param.interval(),
            )?;
        }
        Ok(unknowns)
    }
}

impl AstNode for FnDeclarationNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        if self.should_infer_return_type && self.body.is_none() {
            return Err(type_error("must have a return type or body".to_string())
                .set_interval(self.identifier.interval()));
        }
        if !self.is_anon {
            if self.identifier.token_type == TokenType::Identifier {
                if !can_declare_with_identifier(&self.identifier.clone().literal.unwrap()) {
                    return Err(invalid_symbol(self.identifier.clone().literal.unwrap())
                        .set_interval(self.identifier.interval()));
                }
                if self.is_external && self.identifier.literal.clone().is_some_and(|s| s == "main")
                {
                    return Err(syntax_error(
                        "cannot declare an external main function".to_string(),
                    )
                    .set_interval(self.identifier.interval()));
                }
            } else {
                // check the signature of the operator method
                if self.class_name.is_none() {
                    return Err(syntax_error(
                        "cannot declare operator method outside of class".to_string(),
                    )
                    .set_interval(self.identifier.interval()));
                }

                if self.params.len() != 2 {
                    return Err(type_error(format!(
                        "operator overloading methods must have exactly 2 parameters, found {}",
                        self.params.len()
                    ))
                    .set_interval(self.identifier.interval()));
                }

                // check for no default values
                for param in &self.params {
                    if param.default_value.is_some() {
                        return Err(type_error(
                            "operator overload methods cannot have default values".to_string(),
                        )
                        .set_interval(param.position.clone()));
                    }
                }
            }
        } else {
            // is anonymous function

            if self.class_name.is_some() {
                return Err(syntax_error(
                    "cannot declare anonymous function inside of class".to_string(),
                )
                .set_interval(self.position.clone()));
            }
        }

        self.params_scope = Some(Scope::new_fn_ctx(ctx.clone(), self.is_anon));

        for param in &self.params {
            if param.default_value.is_some() {
                param
                    .default_value
                    .clone()
                    .unwrap()
                    .borrow_mut()
                    .setup(self.params_scope.clone().unwrap())?;
            }
        }
        if self.body.is_some() {
            self.body
                .clone()
                .unwrap()
                .borrow_mut()
                .setup(self.params_scope.clone().unwrap())?;
        }

        self.ret_type
            .borrow_mut()
            .setup(self.params_scope.clone().unwrap())
    }
    fn type_check(&self, mut ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        if self.is_exported {
            let parent = ctx.borrow().get_parent();
            if let Some(parent) = parent {
                ctx = parent;
            }
        }

        let mut unknowns = self.declare_generic_parameters_on_own_scope(ctx.clone())?;

        // don't use param_scope so that the function can have params
        // with the same name as the function
        let ret_type_result = self
            .ret_type
            .borrow()
            .type_check(self.params_scope.clone().unwrap())?;
        unknowns += ret_type_result.unknowns;

        let ret_type = if self.should_infer_return_type {
            mut_rc(UnknownType {})
        } else {
            ret_type_result.t
        };

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
                    .set_interval(position.clone()));
            }

            let mut param_type = None;
            if let Some(type_) = type_ {
                let param_type_res = type_
                    .borrow_mut()
                    .type_check(self.params_scope.clone().unwrap())?;
                param_type = Some(param_type_res.t);
                unknowns += param_type_res.unknowns;
            }
            if let Some(default_value) = default_value.clone() {
                if seen_param_without_default {
                    return Err(type_error(format!(
                        "parameters after `{}` must have default values",
                        identifier
                    ))
                    .set_interval(position.clone()));
                }

                // TODO: type check on global scope
                let default_value_type = default_value
                    .borrow_mut()
                    .type_check(self.params_scope.clone().unwrap())?;
                unknowns += default_value_type.unknowns;

                if param_type.is_none() {
                    param_type = Some(default_value_type.t.clone());
                }
                if !param_type
                    .clone()
                    .unwrap()
                    .borrow()
                    .contains(default_value_type.t.clone())
                {
                    return Err(type_error(format!(
                        "default value for parameter `{}` is not of type `{}`",
                        identifier,
                        param_type.unwrap().borrow().str()
                    ))
                    .set_interval(default_value.borrow_mut().pos()));
                }
            } else {
                seen_param_without_default = true;
            }

            if param_type.is_none() {
                return Err(
                    type_error(format!("parameter `{}` must have a type", identifier))
                        .set_interval(position),
                );
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
            if ctx.borrow().is_frozen() {
                self.params_scope
                    .clone()
                    .unwrap()
                    .borrow_mut()
                    .update_dec_type(&identifier.clone(), param_type.unwrap(), self_pos)?;
            } else {
                self.params_scope.clone().unwrap().borrow_mut().declare(
                    SymbolDec {
                        name: identifier.clone(),
                        id: format!("qword [rbp + {}]", 8 * ((num_params - (i + 1)) + 2)),
                        is_constant: true,
                        is_type: false,
                        is_func: false,
                        require_init: false,
                        is_defined: true,
                        is_param: true,
                        type_: param_type.unwrap().clone(),
                        position: self.position.clone(),
                    },
                    self_pos,
                )?;
            }
        }

        let this_type: MutRc<FnType>;
        if ctx.borrow().is_frozen() {
            let this_type_any = ctx.borrow().get_dec_from_id(&self.id()).type_;
            unsafe {
                // need reference to value in ctx,
                // so can't use 'as_fn()'
                this_type = (&*(&this_type_any as *const dyn Any as *const MutRc<FnType>)).clone();
            }
            // override with latest data
            this_type.borrow_mut().parameters = parameters;
            this_type.borrow_mut().ret_type = ret_type.clone();
        } else {
            let mut generic_args = HashMap::new();
            for generic_param in self.generic_parameters.iter() {
                generic_args.insert(
                    generic_param.literal.clone().unwrap(),
                    mut_rc(GenericType {
                        identifier: generic_param.clone(),
                    }) as MutRc<dyn Type>,
                );
            }

            this_type = mut_rc(FnType {
                id: ctx.borrow_mut().get_id(),
                name: self.id(),
                ret_type: ret_type.clone(),
                parameters,
                generic_args,
                generic_params_order: self.generic_parameters.clone(),
            });
            // declare in the parent context
            ctx.borrow_mut().declare(
                SymbolDec {
                    name: self.id(),
                    id: self.id(),
                    is_constant: true,
                    is_type: false,
                    is_func: true,
                    require_init: !self.is_external,
                    is_defined: self.body.is_some(),
                    is_param: false,
                    type_: this_type.clone(),
                    position: self.pos(),
                },
                self.pos(),
            )?;
        }

        if let Some(ref body) = self.body {
            let TypeCheckRes {
                t: body_ret_type,
                is_returned,
                always_returns,
                unknowns: body_unknowns,
                ..
            } = body
                .borrow()
                .type_check(self.params_scope.clone().unwrap())?;
            unknowns += body_unknowns;

            let mut inferred_ret_type = ret_type;
            if self.should_infer_return_type {
                if !is_returned {
                    return Err(type_error(
                        "must return a value or specify return type".to_string(),
                    )
                    .set_interval(self.identifier.interval()));
                }
                this_type.borrow_mut().ret_type = body_ret_type.clone();
                inferred_ret_type = body_ret_type.clone();
            }

            if is_returned
                && !always_returns
                && !inferred_ret_type.borrow().contains(get_type!(&ctx, "Void"))
            {
                return Err(
                    type_error(format!("'{}' doesn't always return a value", self.id()))
                        .set_interval(self.identifier.interval()),
                );
            }

            let body_ret_type = if is_returned {
                body_ret_type
            } else {
                get_type!(ctx, "Void")
            };
            if !inferred_ret_type.borrow().contains(body_ret_type.clone()) {
                if body_ret_type.borrow().str() == "Void" {
                    return Err(type_error(format!(
                        "`{}` must be annotated with return type",
                        self.identifier.str(),
                    ))
                    .set_interval(self.identifier.interval())
                    .hint(format!(
                        "found type `{}` being returned",
                        inferred_ret_type.borrow().str()
                    )));
                }
                return Err(type_error(format!(
                    "`{}` has return type `{}` but found `{}` being returned",
                    self.identifier.str(),
                    inferred_ret_type.borrow().str(),
                    body_ret_type.borrow().str()
                ))
                .set_interval(self.identifier.interval()));
            }
        }

        Ok(TypeCheckRes::from(this_type, unknowns))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        if !self.has_usage
            && o1_enabled("dead-code-elimination", &ctx.borrow().get_cli_args())
            && self
                .identifier
                .literal
                .clone()
                .map(|s| s != "main")
                .or(Some(true))
                .unwrap()
        {
            // kinda needs attributes before this can be done
            // as some STD functions are 'unused' but still required
            // also, TODO: better system (eg dependency tree)
            //       because this will only prune functions which have
            //       no callers anywhere, when what we want is to prune
            //       functions which can never be called from 'main'
            //return Ok("".to_string());
        }

        if !self.is_anon {
            if let Some(stack_frame) = ctx.borrow_mut().stack_frame_peak() {
                let name = self.identifier.str();
                return Err(type_error(format!(
                    "`{}` is declared inside another function",
                    self.identifier.str()
                ))
                    .set_interval(self.identifier.interval())
                    .hint(format!(
                        "move `{name}` outside of `{}` or make it anonymous with `let {name} = fn () {{ ... }}`",
                        stack_frame.name
                    )));
            }
            if self.body.is_none() {
                return Ok("".to_string());
            }
        }

        let end_label = ctx.borrow_mut().get_anon_label();
        ctx.borrow_mut().stack_frame_push(CallStackFrame {
            name: self.id(),
            ret_lbl: end_label.clone(),
        });

        let body = self
            .body
            .clone()
            .or(Some(mut_rc(PassNode {
                position: self.position.clone(),
            })))
            .unwrap()
            .borrow_mut()
            .asm(self.params_scope.clone().unwrap())?;
        let params_scope = self.params_scope.clone();
        let params_scope = params_scope.unwrap();
        let params_scope = params_scope.borrow_mut();
        let (data_defs, text_defs) = params_scope.get_definitions();

        let text = text_defs
            .iter()
            .map(|k| format!("{}: \n{}", k.name, k.text.as_ref().unwrap()))
            .collect::<Vec<String>>()
            .join("\n");

        let stack_space_for_local_vars = (
            data_defs.len()
            // make sure to maintain 16 byte alignment
            // add 1 only when even number of local vars
            // as we 'push rbp' at the start
            //+ (data_defs.len() % 2 == 1) as usize)
        ) * 8;
        let self_pos = self.pos();

        // As this assumes .asm will only be called once, we need to do a little hack here:
        // When called for the second time, we want to return the label for anon functions,
        // but not redefine the function. Ctx.define will only return an error if the symbol
        // is already defined, so we can just ignore the result.
        let _ = ctx.borrow_mut().define(
            SymbolDef {
                name: self.id(),
                data: None,
                text: Some(format!(
                    "
                        endbr64
                        push rbp
                        mov rbp, rsp
                        {}
                        {body}
                    {end_label}:
                        mov rsp, rbp
                        pop rbp
                        ret
                    {text}
                    ",
                    if stack_space_for_local_vars > 0 {
                        format!("sub rsp, {stack_space_for_local_vars}")
                    } else {
                        "".to_string()
                    }
                )),
            },
            self_pos,
        );

        ctx.borrow_mut().stack_frame_pop();
        if self.is_anon {
            return Ok(format!(
                "
                    lea rax, [rel {}]
                    push rax
                ",
                self.identifier.clone().str()
            ));
        }
        Ok("".to_string())
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}

impl Debug for FnDeclarationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(FnDeclarationNode {:?})", self.position)
    }
}
