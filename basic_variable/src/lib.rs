// region variable
fn variable_without_use() {
    // 未使用的变量用下划线开头
    let _x = 5;
    // 正常变量如果没使用编译器会warn
    let y = 7;
}

struct Example {
    e: i32,
}

fn variable_deconstrution() {
    // 解构声明
    let (a, mut b): (bool, bool) = (true, false);

    println!("a = {:?}, b= {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 解构赋值
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5, 6, 7];

    Example { e, .. } = Example { e: 5 };

    assert_eq!([1, 2, 1, 6, 5], [a, b, c, d, e]);
}

fn constant() {
    const PI: f32 = 3.1415926;

    assert_eq!(3.1415926, PI)
}

fn variable_shadowing() {
    let x = 5;
    let x = x + 1;
    println!("x shadowing the declaration before {}", x);

    {
        // 局部作用域的遮蔽
        let x = x * 2;
        println!("local scope shadowing {}", x);
    }

    // 这里还是外部作用域的x
    println!("outer scope x {}", x);

    // 但是遮蔽只能适用于相同类型
    // 下面这个例子会报错
    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!(spaces)
}

// endregion

// region types
fn test_integer() {
    let mut i1: i8 = 127;
    println!("{}", i1);
    // 溢出 （debug模式下运行时错误, release模式下按照循环溢出规则处理，类似于C）
    // i1 = i1 + 1;
    // println!("{}", i1);

    let _i2: i16 = 32767;
    let _u2: u16 = 65535;
    let _i3: i32 = 1;
    let _i4: i64 = 1;
    let _i5: i128 = 1;
    let _i6: isize = 1; // 在现代机器上都是64位
    let _u6: usize = 1; // 在现代机器上都是64位

    // 默认是i32类型
    let _default_int = 1;
    // 也可以通过后缀方式注明类型
    let _typed_int = 1u128;

    // 整数字面量形式
    let _decimal = 100_000;
    let _hex = 0xff; // -> 255
    let _oct = 0o0; // -> 0
    let _binary = 0b1111_0000;
    let _byte = b'A'; // -> 65
    println!("{}", _byte);
}

fn test_float() {
    let _f1: f32 = 0.1;
    let _f2: f64 = 0.1;

    // 浮点数默认是 f64 类型
    let x = 2.0;

    // 数值类型有对应的方法
    println!("{}", _f1.round())

    // 浮点数不能实现 std::cmp::Eq 的trait
    // 这意味着浮点数不能做相等比较， 也不能作为HashMap的key
    // 下面的代码会报编译错误
    // 0.1 + 0.2 == 0.3
}

fn test_char() {
    // 字符用单引号''
    // unicode都是字符
    let c: char = 'z';
    let emoji = '👻';
    let nin = '您';

    println!("{}, {}, {}", c, emoji, nin);
    // 字符通通占用4个字节
    println!("size of 'z':{}", std::mem::size_of_val(&c));
    println!("size of '👻':{}", std::mem::size_of_val(&emoji));
    println!("size of '您':{}", std::mem::size_of_val(&nin));
}

fn test_bool() {
    let _t = true;
    let _f: bool = false;
}

fn test_unit_type() {
    let _a = ();
}
// endregion

// region expression
fn test_expression() {
    // let一定是语句（至少目前的rust版本是这样）
    // 表达式可以作为右值

    // 语句块也可以是表达式
}
// endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable() {
        variable_without_use();
        variable_deconstrution();
        constant();
        variable_shadowing();
    }

    #[test]
    fn types() {
        test_integer();
        test_float();
        test_char();
        test_bool();
        test_unit_type();
    }
}
