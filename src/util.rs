use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub type MutRc<T> = Rc<RefCell<T>>;

pub fn new_mut_rc<T>(t: T) -> MutRc<T> {
    Rc::new(RefCell::new(t))
}

// pub fn keys_match<T: Eq + Hash, U, V>(map1: &HashMap<T, U>, map2: &HashMap<T, V>) -> bool {
//     map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
// }

pub fn intersection<T: Eq + Hash + Clone, U, V>(
    map1: &HashMap<T, U>,
    map2: &HashMap<T, V>,
) -> (Vec<T>, Vec<T>, Vec<T>) {
    let mut in_both = Vec::new();
    let mut in_first = Vec::new();
    let mut in_second = Vec::new();
    for (k, _) in map1 {
        if map2.contains_key(k) {
            in_both.push(k.clone());
        } else {
            in_first.push(k.clone());
        }
    }
    for (k, _) in map2 {
        if !map1.contains_key(k) {
            in_second.push(k.clone());
        }
    }
    (in_first, in_both, in_second)
}

#[macro_export]
macro_rules! strings_vec {
    ($($x:expr),*$(,)?) => (vec![$($x.to_string()),*]);
    () => (vec![]);
}
