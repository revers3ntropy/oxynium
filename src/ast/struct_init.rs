use crate::ast::types::Type;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error, unknown_symbol};
use crate::position::Interval;
use crate::util::{intersection, MutRc};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct StructInitNode {
    pub identifier: String,
    pub fields: Vec<(String, MutRc<dyn Node>)>,
    pub position: Interval,
}

impl StructInitNode {
    fn field_types_hashmap(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<HashMap<String, Rc<dyn Type>>, Error> {
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

impl Node for StructInitNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = String::new();

        let mut fields = self.fields.clone();
        fields.sort_by(|a, b| a.0.cmp(&b.0));

        let field_asm = self.field_asm_hashmap(ctx.clone())?;

        for (name, _) in fields.iter() {
            asm.push_str(&format!("{}\n", field_asm[name]));
        }

        asm.push_str(&format!(
            "
            mov rdi, {}
            call malloc WRT ..plt
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

        asm.push_str(&format!("
            push rax
        "));

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
        let struct_type = type_.as_struct();
        if struct_type.is_none() {
            return Err(type_error(format!(
                "{} is not a struct",
                self.identifier
            )));
        }
        let struct_type = struct_type.unwrap();

        let instance_fields_hashmap = self.field_types_hashmap(ctx.clone())?;

        let mut type_fields_hashmap = HashMap::new();
        for field in struct_type.fields.clone() {
            type_fields_hashmap.insert(field.name, field.type_.clone());
        }

        let (extra, fields, missing) =
            intersection(&instance_fields_hashmap, &type_fields_hashmap);

        if extra.len() > 0 {
            return Err(type_error(format!(
                "Unknown fields in struct initialization: {}",
                extra
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            )));
        }

        if missing.len() > 0 {
            return Err(type_error(format!(
                "Missing fields in struct initialization: {}",
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
                .contains(instance_fields_hashmap.get(&field).unwrap().clone())
            {
                return Err(type_error(format!(
                    "Type mismatch in struct initialization field '{field}': Expected {} but found {}",
                    type_fields_hashmap.get(&field).unwrap().str(),
                    instance_fields_hashmap.get(&field).unwrap().str(),
                )));
            }
        }

        Ok((type_, None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
