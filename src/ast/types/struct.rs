use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::ast::types::Type;

#[derive(Clone, Debug)]
pub struct StructFieldType {
    pub name: String,
    pub type_: Rc<dyn Type>
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

impl Debug for StructType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Type for StructType {
    fn is_ptr(&self) -> bool { true }
    fn str(&self) -> String {
        format!(
            "Struct {} {{ {} }}",
            self.name,
            self.fields.iter()
                .map(|p| p.str())
                .collect::<Vec<String>>()
                .join("; ")
        )
    }

    fn contains(&self, t: Rc<dyn Type>) -> bool {
        std::ptr::eq(t.as_ref(), self)
    }

    fn as_struct(&self) -> Option<StructType> {
        Some(self.clone())
    }
}
