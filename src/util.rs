use crate::error::{io_error, Error};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::rc::Rc;

pub type MutRc<T> = Rc<RefCell<T>>;

pub fn new_mut_rc<T>(t: T) -> MutRc<T> {
    Rc::new(RefCell::new(t))
}

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

pub fn num_digits(mut n: i64) -> usize {
    if n == 0 {
        return 1;
    }
    if n < 0 {
        unreachable!();
    }
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

pub fn indent(s: String, indent: usize) -> String {
    let mut res = String::new();
    for line in s.lines() {
        if !line.is_empty() {
            res.push_str(&" ".repeat(indent));
            res.push_str(line);
        }
        res.push_str("\n");
    }
    res[..res.len() - 1].to_string()
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let input_file = File::open(path);
    if input_file.is_err() {
        return Err(io_error(format!("Failed to open file '{}'", path)));
    }

    let mut input = String::new();
    let read_file_result = input_file.unwrap().read_to_string(&mut input);
    if read_file_result.is_err() {
        return Err(io_error(format!(
            "Failed to read file '{}': {}",
            path,
            read_file_result.err().unwrap()
        )));
    }

    Ok(input)
}

pub unsafe fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[macro_export]
macro_rules! strings_vec {
    ($($x:expr),*$(,)?) => (vec![$($x.to_string()),*]);
    () => (vec![]);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn intersection() {
        let map1 = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
        let map2 = HashMap::from([("a", 1), ("b", 2), ("d", 4)]);
        let (in_first, mut in_both, in_second) = super::intersection(&map1, &map2);
        assert_eq!(in_first, vec!["c"]);
        in_both.sort();
        assert_eq!(in_both, vec!["a", "b"]);
        assert_eq!(in_second, vec!["d"]);
    }

    #[test]
    fn num_digits() {
        assert_eq!(super::num_digits(0), 1);
        assert_eq!(super::num_digits(1), 1);
        assert_eq!(super::num_digits(9), 1);
        assert_eq!(super::num_digits(10), 2);
        assert_eq!(super::num_digits(99), 2);
        assert_eq!(super::num_digits(100), 3);
        assert_eq!(super::num_digits(999), 3);
        assert_eq!(super::num_digits(1000), 4);
        assert_eq!(super::num_digits(100000000000000), 15);
        assert_eq!(super::num_digits(999999999999999), 15);
    }

    #[test]
    fn indent() {
        assert_eq!(
            super::indent("a\n\nb\n\nc".to_string(), 2),
            "  a\n\n  b\n\n  c"
        );
    }
}
