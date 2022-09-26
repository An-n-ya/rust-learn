fn basic_match() {
    enum Direction {
        East,
        West,
        North,
        South,
    }
    // match 需要穷举所有可能
    // 每个分支需要是表达式，且返回值统一
    // match本身也是表达式
    let dir = Direction::East;
    let res = match dir {
        Direction::East => "East",
        Direction::North | Direction::South => "North or South",
        _ => "West", // 这里的 _ 是穷尽匹配，能匹配剩下的所有情况，类似于其他语言的default
    };
    println!("{}", res);

    // match是代替连续else if的好方案
}

fn match_enum_with_val() {
    // 对于有val的enum，也可以使用enum
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 0, 0),
    ];
    for action in actions {
        // 每个分支看起来就像一个函数一样
        match action {
            Action::Say(s) => println!("{}", s),
            Action::MoveTo(x, y) => println!("point from (0,0) move to ({} {})", x, y),
            Action::ChangeColorRGB(r, g, b) => {
                println!("change color into (r:{}, g:{}, b:{})", r, g, b)
            }
        }
    }
}

fn test_if_let() {
    // 对于像 Option::Some 这样的值使用match时，只需要处理两个分支
    // 对于这样的模式，可以使用if let简化
    let v = Some(3u8);
    if let Some(x) = v {
        // 这里的x是局部变量
        println!("{}", x)
    }
}

fn test_matches() {
    let foo = 'f';
    // 通过matches！宏得到布尔值，在filter()方法中很常用
    println!("{}", matches!(foo, 'A'..='Z' | 'a'..='z'));
}

fn test_option_match() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn option_test() {
        test_option_match();
    }

    #[test]
    fn match_test() {
        basic_match();
        match_enum_with_val();
        test_if_let();
        test_matches();
    }
}
