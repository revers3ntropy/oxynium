use std::collections::HashMap;
use crate::ast::types::Type;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::util::{intersection};

#[derive(Clone, Debug)]
pub struct StructFieldType {
    pub name: String,
    pub type_: Rc<dyn Type>,
}
impl StructFieldType {
    fn str(&self) -> String {
        if self.name == "" {
            self.type_.str()
        } else {
            format!("{}: {}", self.name, self.type_.str())
        }
    }
}

#[derive(Clone)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<StructFieldType>,
}

impl StructType {
    pub fn field_type(&self, field: &str) -> Option<Rc<dyn Type>> {
        self.fields
            .iter()
            .find(|f| f.name == field)
            .map(|f| f.type_.clone())
    }

    pub fn field_offset(&self, field: String) -> usize {
        self.fields
            .iter()
            .position(|f| f.name == field)
            .unwrap()
            * 8
    }
    fn field_types_hashmap(&self) -> HashMap<String, Rc<dyn Type>> {
        let mut instance_fields_hashmap = HashMap::new();
        for field in self.fields.clone() {
            instance_fields_hashmap.insert(
                field.name,
                field.type_,
            );
        }
        instance_fields_hashmap
    }
}

impl Debug for StructType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Type for StructType {
    fn is_ptr(&self) -> bool {
        true
    }
    fn str(&self) -> String {
        format!(
            "Struct {} {{ {} }}",
            self.name,
            self.fields
                .iter()
                .map(|p| p.str())
                .collect::<Vec<String>>()
                .join("; ")
        )
    }

    fn contains(&self, t: Rc<dyn Type>) -> bool {
        let t = t.as_struct();
        if t.is_none() {
            return false;
        }
        let t = t.unwrap();

        let other_fields_hashmap = t.field_types_hashmap();
        let self_fields_hashmap = self.field_types_hashmap();
        let (extra, fields, missing) =
            intersection(&self_fields_hashmap, &other_fields_hashmap);

        if extra.len() > 0 || missing.len() > 0 {
            return false
        }

        for field in fields {
            if !self_fields_hashmap
                .get(&field)
                .unwrap()
                .contains(other_fields_hashmap.get(&field).unwrap().clone())
            {
                return false;
            }
        }
        true
    }

    fn as_struct(&self) -> Option<StructType> {
        Some(self.clone())
    }
}
