use std::fmt::{Display, Formatter};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    // ( 1.1 1.2 )
    // ( 2.1 2.2 )
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );
        println!("长元组的第一个值：{}", long_tuple.0);
        println!("长元组的第二个值：{}", long_tuple.1);

        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        println!("元组的元组：{:?}", tuple_of_tuples);

        let pair = (1, true);
        println!("对组是 {:?}", pair);

        println!("反转后的对组是 {:?}", reverse(pair));

        println!("单元素元组：{:?}", (5u32,));
        println!("仅是一个整数：{:?}", (5u32));

        let tuple = (1, "hello", 4.5, true);

        let (a, b, c, d) = tuple;
        println!("{:?}、{:?}、{:?}、{:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("矩阵：\n{}", matrix);
        println!("转置：\n{}", transpose(matrix));
    }
}
