fn test_if() {
    let condition = true == true;
    // 花括号不能省略
    if condition {
    } else {
    }
    // 类三元表达式
    let number = if condition { 10 } else { 11 };
    println!("{}", number);
}

fn test_for() {
    // for item in collection       转移所有权
    // for item in &collection      不可变借用
    // for item in &mut collection  可变借用
    let mut collection = ["a", "b", "c"];
    // 建议直接使用迭代器方式遍历，不要用索引方式遍历，原因如下：
    // 使用索引时会进行"边界检查", 而迭代器不需要，所以使用迭代器效率更高
    // 如果遍历时collection发生变化，使用索引方式可能会有"脏读"
    for item in collection {
        println!("{}", item);
    }

    // 也可以通过enumerate获得循环时的索引
    for (i, v) in collection.iter().enumerate() {
        println!("{}: {}", i, v);
    }

    // 循环十次的简化写法
    for _ in 0..10 {}

    let mut i = 0;
    while i < 10 {
        i += 1;
    }

    // 无条件循环
    loop {
        println!("这里是无线循环");
        break;
    }

    // loop 是一个表达式，可以使用break返回一个值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_test() {
        test_if();
    }

    #[test]
    fn loop_test() {
        test_for();
    }
}
