use std;
use std::fmt;
use std::fmt::Formatter;

pub fn literal() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("true AND false is {}", true && false);

    println!("1 << 5 is {}", 1u32 << 5);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "⌈{} {}⌉\n", self.0, self.1)?;
        write!(f, "⌊{} {}⌋", self.2, self.3)
    }
}

fn transpose(mat: Matrix) -> Matrix {
    Matrix(mat.0, mat.2, mat.1, mat.3)
}

pub fn tuple() {
    // 元组可以包含不同的类型
    let various_tuple = (1u8, 0.2f32, true, 'a');

    println!(
        "{}, {}, {}",
        various_tuple.0, various_tuple.2, various_tuple.3
    );

    // 元组可以嵌套
    let tuple_of_tuples = ((1u8, 2u16), (4i8, 999u64), 0.1f32);
    println!("{:?}", tuple_of_tuples);
    // 使用{:?}最多只能打印长度为12的元组

    // 元组可以解包
    let (a, b, c) = tuple_of_tuples;
    println!("{:?} {:?} {}", a, b, c);

    let mat = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", mat);
    println!("Transpose:\n{}", transpose(mat));
}

fn slice_test(slice: &[i32]) {
    println!("the first element of the slice is: {}", slice[0]);
}

pub fn array_and_slice() {
    // 数组一组拥有相同类型T且在内存中连续存储的对象的集合
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 初始化为相同的值
    let zeros: [i32; 10] = [0; 10];
    println!("arr: {:?}, zeros: {:?}", arr, zeros);
    println!("arr_size: {}", zeros.len());

    // 切片不能直接打印
    // println!("slice: {:?}", arr[1..2]);

    // 切片可以作为参数传递
    slice_test(&arr[2..4])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        println!("=====literal=====");
        literal();
    }

    #[test]
    fn test_tuple() {
        println!("=====tuple=====");
        tuple();
    }

    #[test]
    fn test_array_and_slice() {
        println!("=====array and slice=====");
        array_and_slice();
    }
}
