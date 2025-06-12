// [0: 1, 1: 2, 2: 3]

use std::fmt::{Display, Formatter};

#[allow(dead_code)]
struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (idx, item) in self.0.iter().enumerate() {
            if idx > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", idx, item)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = List(vec![1, 2, 3]);
        println!("{}", list);
    }
}
