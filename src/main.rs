use std::fmt;
use std::fmt::{write, Formatter};

// 实现fmt::Debug trait从而支持std::fmt格式化打印
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct MinMax(i64, i64);

// 实现fmt::Display
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    println!("{} days", 31);
    println!("{} days", 31i64); // 通过后缀改变类型

    // 在模板字符串中使用索引
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 也可以使用命名参数
    println!(
        "{year}-{month}-{day}",
        year = "2022",
        month = "Sept",
        day = "20"
    );

    // 格式化参数
    println!("pi is {:.2}", 3.1415926);

    // 打印结构体
    println!("now structure is {:?}", Structure(3));

    // 美化打印
    println!(
        "{:#?}",
        Person {
            name: "Peter",
            age: 27
        }
    );

    // 实现Display后打印
    let minmax = MinMax(0, 1);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let s = String::from("hello");
    println!("s: {}", s);

    // List 打印测试
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
