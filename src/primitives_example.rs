#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[allow(dead_code, unused)]
    #[test]
    fn test() {
        let logical: bool = true;

        let a_float: f64 = 1.0;
        let an_integer = 5i32;

        let default_float = 3.0;
        let default_integer = 7;

        let mut inferred_type = 12;
        inferred_type = 4294967296i64;

        let mut mutable = 12;
        mutable = 21;

        let mutable = true;

        let my_arr: [i32; 5] = [1, 2, 3, 4, 5];

        let my_tupple: (i32, f64, bool) = (1, 2.0, true);
    }
}
