use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::Type;
use crate::util::{intersection, mut_rc, MutRc};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ClassInitNode {
    pub identifier: Token,
    pub fields: Vec<(String, MutRc<dyn AstNode>)>,
    pub position: Interval,
    pub generic_args: Vec<MutRc<dyn AstNode>>,
}

impl ClassInitNode {
    fn field_types_hashmap(
        &self,
        ctx: MutRc<dyn Context>,
    ) -> Result<(HashMap<String, MutRc<dyn Type>>, usize), Error> {
        let mut unknowns = 0;
        let mut instance_fields_hashmap = HashMap::new();
        for field in self.fields.clone() {
            let field_type_res = field.1.borrow().type_check(ctx.clone())?;
            unknowns += field_type_res.unknowns;
            instance_fields_hashmap.insert(field.0, field_type_res.t);
        }
        Ok((instance_fields_hashmap, unknowns))
    }
    fn field_asm_hashmap(&self, ctx: MutRc<dyn Context>) -> Result<HashMap<String, String>, Error> {
        let mut instance_fields_hashmap = HashMap::new();
        for field in self.fields.clone() {
            instance_fields_hashmap.insert(field.0, field.1.borrow_mut().asm(ctx.clone())?);
        }
        Ok(instance_fields_hashmap)
    }
}

impl AstNode for ClassInitNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        for field in self.fields.clone() {
            field.1.borrow_mut().setup(ctx.clone())?;
        }
        for generic_arg in self.generic_args.clone() {
            generic_arg.borrow_mut().setup(ctx.clone())?;
        }
        Ok(())
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        if !ctx
            .borrow()
            .has_dec_with_id(&self.identifier.clone().literal.unwrap())
        {
            if ctx.borrow().throw_on_unknowns() {
                return Err(unknown_symbol(format!(
                    "class '{}'",
                    self.identifier.clone().literal.unwrap(),
                ))
                .set_interval(self.identifier.interval()));
            }
            for arg in self.generic_args.clone() {
                unknowns += arg.borrow().type_check(ctx.clone())?.unknowns;
            }
            for field in self.fields.clone() {
                unknowns += field.1.borrow().type_check(ctx.clone())?.unknowns;
            }
            return Ok(TypeCheckRes::unknown_and(unknowns));
        }

        let class_type_raw = ctx
            .borrow()
            .get_dec_from_id(&self.identifier.clone().literal.unwrap())
            .type_
            .clone();
        let class_type = class_type_raw.borrow().as_class();
        if class_type.is_none() {
            return Err(type_error(format!(
                "`{}` is not a class",
                self.identifier.clone().literal.unwrap()
            )));
        }
        let mut class_type = class_type.unwrap();

        if self.generic_args.len() != class_type.generic_params_order.len() {
            return Err(type_error(format!(
                "class `{}` takes {} generic arguments, but {} were given",
                self.identifier.clone().literal.unwrap(),
                class_type.generic_params_order.len(),
                self.generic_args.len()
            ))
            .set_interval(self.identifier.interval()));
        }

        let mut generics = HashMap::new();
        let mut i = 0;
        for arg in self.generic_args.clone() {
            let arg_type_res = arg.borrow().type_check(ctx.clone())?;
            unknowns += arg_type_res.unknowns;
            let name = class_type.generic_params_order[i].clone();
            generics.insert(name.literal.unwrap().clone(), arg_type_res.t);
            i += 1;
        }

        class_type = class_type.clone();
        class_type.generic_args = generics.clone();

        class_type = class_type
            .concrete(&HashMap::new(), &mut HashMap::new())?
            .borrow()
            .as_class()
            .unwrap();

        let (instance_fields_hashmap, field_unknowns) = self.field_types_hashmap(ctx.clone())?;
        unknowns += field_unknowns;

        let mut type_fields_hashmap = HashMap::new();
        for (name, field) in class_type.fields.clone() {
            type_fields_hashmap.insert(name, field.type_.clone());
        }

        let (extra, fields, missing) = intersection(&instance_fields_hashmap, &type_fields_hashmap);

        if extra.len() > 0 {
            return Err(type_error(format!(
                "unknown fields for class `{}`: {}",
                class_type.str(),
                extra
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .set_interval(self.pos()));
        }

        if missing.len() > 0 {
            return Err(type_error(format!(
                "missing fields for class `{}`: {}",
                class_type.str(),
                missing
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .set_interval(self.pos()));
        }

        for field in fields {
            if !type_fields_hashmap
                .get(&field)
                .unwrap()
                .borrow()
                .contains(instance_fields_hashmap.get(&field).unwrap().clone())
            {
                return Err(type_error(format!(
                    "type mismatch in class initialization field `{field}`, expected `{}` but found `{}`",
                    type_fields_hashmap.get(&field).unwrap().borrow().str(),
                    instance_fields_hashmap.get(&field).unwrap().borrow().str(),
                )).set_interval(self.pos()));
            }
        }

        Ok(TypeCheckRes::from(mut_rc(class_type), unknowns))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let mut asm = String::new();

        let mut fields = self.fields.clone();

        let field_asm = self.field_asm_hashmap(ctx.clone())?;

        let class_type = ctx
            .borrow()
            .get_dec_from_id(&self.identifier.clone().literal.unwrap())
            .type_
            .clone()
            .borrow()
            .as_class()
            .unwrap();
        let is_primitive = class_type.is_primitive;

        fields.sort_by(|a, b| {
            class_type
                .field_offset(a.0.clone())
                .cmp(&class_type.field_offset(b.0.clone()))
        });

        let mut fields_asm_iter = field_asm.iter().collect::<Vec<(&String, &String)>>();

        fields_asm_iter.sort_by(|a, b| {
            class_type
                .field_offset(a.0.clone())
                .cmp(&class_type.field_offset(b.0.clone()))
        });
        fields_asm_iter.reverse();

        for (name, _) in fields_asm_iter {
            asm.push_str(&format!("{}\n", field_asm[name]));
        }

        if fields.len() < 1 {
            return Ok(if is_primitive {
                format!("\n push 0 \n")
            } else {
                format!(
                    "
                        mov rdi, 8
                        mov rsi, 1
                        call _LibC.calloc
                        mov qword [rax], 0
                        push rax
                    "
                )
            });
        }

        asm.push_str(&format!(
            "
                mov rdi, {}
                mov rsi, 8
                call _LibC.calloc
            ",
            fields.len()
        ));

        for i in 0..fields.len() {
            asm.push_str(&format!(
                "
                    pop rdx
                    mov qword [rax + {}], rdx
                ",
                class_type.field_offset(fields[i].0.clone())
            ));
        }

        if is_primitive {
            asm.push_str(&format!("push qword [rax]\n"));
        } else {
            asm.push_str(&format!("push rax\n"));
        }

        Ok(asm)
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
