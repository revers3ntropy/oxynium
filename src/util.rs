use std::cell::RefCell;
use std::rc::Rc;

pub type MutRc<T> = Rc<RefCell<T>>;

pub fn new_mut_rc<T>(t: T) -> MutRc<T> {
    Rc::new(RefCell::new(t))
}

#[macro_export]
macro_rules! strings_vec {
    ($($x:expr),*$(,)?) => (vec![$($x.to_string()),*]);
    () => (vec![]);
}