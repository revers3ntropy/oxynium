use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::position::Interval;
use crate::types::Type;
use crate::util::{intersection, MutRc};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ClassInitNode {
    pub identifier: String,
    pub fields: Vec<(String, MutRc<dyn Node>)>,
    pub position: Interval,
}

impl ClassInitNode {
    fn field_types_hashmap(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<HashMap<String, MutRc<dyn Type>>, Error> {
        let mut instance_fields_hashmap = HashMap::new();
        for field in self.fields.clone() {
            instance_fields_hashmap.insert(
                field.0,
                field.1.borrow_mut().type_check(ctx.clone())?.0,
            );
        }
        Ok(instance_fields_hashmap)
    }
    fn field_asm_hashmap(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<HashMap<String, String>, Error> {
        let mut instance_fields_hashmap = HashMap::new();
        for field in self.fields.clone() {
            instance_fields_hashmap
                .insert(field.0, field.1.borrow_mut().asm(ctx.clone())?);
        }
        Ok(instance_fields_hashmap)
    }
}

impl Node for ClassInitNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = String::new();

        let mut fields = self.fields.clone();
        fields.sort_by(|a, b| a.0.cmp(&b.0));

        let field_asm = self.field_asm_hashmap(ctx.clone())?;

        for (name, _) in fields.iter() {
            asm.push_str(&format!("{}\n", field_asm[name]));
        }

        let class_type = ctx
            .borrow_mut()
            .get_dec_from_id(&self.identifier)?
            .type_
            .clone()
            .borrow()
            .as_class()
            .unwrap();
        let is_primitive = class_type.is_primitive;

        if fields.len() > 0 {
            asm.push_str(&format!(
                "
                push {}
                call _$_allocate
                pop rcx
            ",
                fields.len() * 8
            ));

            for i in 0..fields.len() {
                asm.push_str(&format!(
                    "
                pop rdx
                mov qword [rax + {}], rdx
            ",
                    (fields.len() - i - 1) * 8
                ));
            }
        } else {
            asm.push_str(&format!(
                "
                mov rdi, 8
                call malloc WRT ..plt
                mov qword [rax], 0
            "
            ));
        }

        if is_primitive {
            asm.push_str(&format!(
                "
                push qword [rax]
            "
            ));
        } else {
            asm.push_str(&format!(
                "
                push rax
            "
            ));
        }

        Ok(asm)
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone())
                .set_interval(self.position.clone()));
        }
        let type_ = ctx
            .borrow_mut()
            .get_dec_from_id(&self.identifier)?
            .type_
            .clone();
        let class_type = type_.borrow().as_class();
        if class_type.is_none() {
            return Err(type_error(format!(
                "{} is not a class",
                self.identifier
            )));
        }
        let class_type = class_type.unwrap();

        let instance_fields_hashmap = self.field_types_hashmap(ctx.clone())?;

        let mut type_fields_hashmap = HashMap::new();
        for field in class_type.fields.clone() {
            type_fields_hashmap.insert(field.name, field.type_.clone());
        }

        let (extra, fields, missing) =
            intersection(&instance_fields_hashmap, &type_fields_hashmap);

        if extra.len() > 0 {
            return Err(type_error(format!(
                "Unknown fields in class initialization: {}",
                extra
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            )));
        }

        if missing.len() > 0 {
            return Err(type_error(format!(
                "Missing fields in class initialization: {}",
                missing
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            )));
        }

        for field in fields {
            if !type_fields_hashmap
                .get(&field)
                .unwrap()
                .borrow()
                .contains(instance_fields_hashmap.get(&field).unwrap().clone())
            {
                return Err(type_error(format!(
                    "Type mismatch in class initialization field '{field}': Expected {} but found {}",
                    type_fields_hashmap.get(&field).unwrap().borrow().str(),
                    instance_fields_hashmap.get(&field).unwrap().borrow().str(),
                )));
            }
        }

        Ok((type_, None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
