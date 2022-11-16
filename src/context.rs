use std::collections::{ HashMap };


pub(crate) struct Context {
    pub(crate) declarations: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            declarations: HashMap::new(),
        }
    }

    pub fn declare(&mut self, ty: String) -> String {
        // generate random name
        let name = format!("__data_{}", rand::random::<u32>());
        self.declarations.insert(name.clone(), ty);
        name
    }
}