use std::mem;
use std::ptr::null_mut;

// region 字符串

fn test_string() {
    // &str 与 String 类型不同
    // rust中字符串字面量的类型默认是 &str
    let name = "Ankh";
    let name2 = String::from("Ankh");

    // &str 是切片， 与Go中的切片很接近
    // 字符串可以得出切片
    // 使用 .. 这个 range 序列语法
    let _nn = &name2[1..3];
    // 前后边界有默认值
    let _name3 = &name2[..];

    // rust中字符是Unicode编码，都是4字节   与之对比的是 Java使用utf16作为字符编码
    // 但是字符串使用utf8编码，占用字节数是变化的（1-4字节）
    let uni = '中';
    let utf8 = "中";
    println!(
        "size of char {}, size of string {}",
        mem::size_of_val(&uni),
        mem::size_of_val(utf8)
    ); // 4 和 3

    // rust在语言级别上只有 str 类型
    // 但是在标准库中有 String 类型， 这也是使用最广泛的字符串类型

    // str会硬编码到可执行文件，不能改变长度，位于栈的数据结构
    // String是可变长的, 位于堆的数据结构

    // &str 到 String
    let str_to_string = String::from(name);
    // String 到 &str
    let string_to_str = &name2[..];
    println!("{:?}", (str_to_string, string_to_str));

    // 字符串可以用数字取切片  但不能用数字索引
    // let a = name[1];
    // 但是切片是按照字节来进行的，所以当索引刚好落在utf8编码的中间，就会报错
}

fn test_slice() {
    // 数组也可以有切片
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
    // 类型是 &[i32]

    // 下面的会报运行时错误
    // 越界
    // let ss = &a[1..10];
}

fn test_alter_string() {
    // str类型的字符串是不可以更改的
    // String是可变字符串，可以进行更改

    // String上的操作都是有副作用的， 需要变量是可变的
    let mut s = String::from("hello");
    s.push_str(" r");
    s.push('u');
    s.insert(s.len(), 's');
    s.insert_str(s.len(), "t!");
    println!("{}", s);

    // replace 系列方法都是返回新字符串，没有副作用
    let mut string_pop = String::from("rust 真厉害！");

    // 删除相关的方法都有副作用
    // pop()方法会做utf8到unicode的转换
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    string_pop.pop();
    dbg!(&string_pop);
    // remove() 方法是按照字节处理，如果边界有问题会发生错误
    string_pop.remove(0);

    // 连接操作
    // + +=符号用于字符串连接 相当于调用add() 方法
    // add() 方法第二个参数是引用类型， 因此+运算符的第二个参数应该是切片引用
    let ss = String::from("厉害");
    let result = string_pop + &ss;
    // 字面量直接是切片，所以可以直接加
    let mut result = result + "!";
    result += "!!!";
    println!("{}", result);
    // string_pop 的所有权转移到了add()方法里，并在这个方法里释放了，所以在外边访问不了了
    // println!("{}", string_pop); // 这句报错

    // 如果不想发生所有权转移， 可以使用format! 宏连接字符串
    let rust = format!("{}{}", "r", result);
    println!("{} \n {}", rust, result);
}

// endregion

// region 元组

fn test_tuple() {
    // 多个类型的数据用括号组合在一起就是元组
    // 元组支持协变（variance）即一个元组中允许有多种类型
    // 元组也支持嵌套
    let tup = (1, 0.1, "asb", (1, 'c'));

    // 元组可以解构
    let (x, y, z, (a, b)) = tup;
    println!("{:?}", (x, y, z, a, b));

    // 用数字索引
    println!("{}", tup.0)

    // 元组无法获取长度，无法遍历
}

// endregion

// region 结构体

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn user_factory(email_in: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email_in),
        sign_in_count: 1,
    }
}

fn test_struct() {
    let u1 = user_factory("aa@cc.cc", "aa");
    dbg!(&u1);

    // 访问属性
    println!("{:?}", (u1.email, u1.username, u1.active, u1.sign_in_count));

    // 修改结构体需要mut关键词
    // u1 是不可变变量，因此只有active sign_in_count 可以直接赋值，其他两个属性会涉及所有权，需要把u1变成mut才可以用
    let mut u2 = User {
        email: String::from("ni@nn.cc"),
        username: String::from("ni"),
        ..u1
    };
    u2.username = String::from("ankh");
    println!("{:?}", u2);

    // 结构体中使用引用数据类型需要引入"生命周期"
}

fn test_tuple_struct() {
    #[derive(Debug)]
    struct Tree(i32, *mut Tree, *mut Tree);

    let mut left = Tree(1, null_mut(), null_mut());
    let node = Tree(10, &mut left, null_mut());

    println!("{:?}", node);
}

// endregion

// region 枚举

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
enum PokerVal {
    Num(u8),
    Other(char),
}

#[derive(Debug)]
enum PokerSuitWithVal {
    Clubs(PokerVal),
    Spades(PokerVal),
    Diamonds(PokerVal),
    Hearts(PokerVal),
}

fn test_enum() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    fn print_enum(card: PokerSuit) {
        println!("{:?}", card);
    }

    print_enum(heart);
    print_enum(diamond);

    let heart3 = PokerSuitWithVal::Hearts(PokerVal::Num(3));
    let clubsA = PokerSuitWithVal::Clubs(PokerVal::Other('A'));
    println!("{:?}", (heart3, clubsA));

    // rust 没有多态的特性， 在一定程度上可以使用 Enum 来承担多态的责任
}

fn test_option_enum() {
    let x = Some(5);
    let y = Some(8);
    // let sum = x + y; // Option不能直接参与运算， 需要unwrap

    // Option提供了很多 unwrap 的方法，这里使用了相对简单的 unwrap_or_default
    let sum = x.unwrap_or_default() + y.unwrap_or_default();
    println!("{}", sum);

    // Option::None 需要指明类型
    let t: Option<i32> = None;

    // 另一种常见的方法是使用match
    let unwrapped_val = match t {
        None => 0,
        Some(i) => i,
    };
    println!("{}", unwrapped_val);
}

// endregion

// region 数组

fn test_array() {
    // rust 的数组是基本类型，长度不可变，存储在栈上
    // rust 的Vector数组是动态数组，长度可以变化，存储在堆上
    let months = [
        "January", "February", "March", "April", "May", "June", "July", "October", "November",
        "December",
    ];
    // 数组的类型为 [type; capacity]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", (a, months));
    // 数组初始化
    let a = [3; 10];
    println!("{:?}", a);

    // python会做数组越界检查（运行时）
    // a[10] error

    // 数组也有切片
    let slice = &a[1..3];
    // 切片类型为 &[type]
    println!("{:?}", slice);

    // 数组可以用迭代器遍历
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for n in a.iter() {
        print!("\t{} + 10 = {}\n", n, n + 10);
    }

    // 二维数组
    let arrays = [[0; 10]; 10];
    println!("{:?}", arrays);
}

// endregion
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        test_string();
        test_slice();
        test_alter_string();
    }

    #[test]
    fn tuple() {
        test_tuple();
    }

    #[test]
    fn struct_test() {
        test_struct();
        test_tuple_struct();
    }

    #[test]
    fn enum_test() {
        test_enum();
        test_option_enum();
    }

    #[test]
    fn array() {
        test_array();
    }
}
