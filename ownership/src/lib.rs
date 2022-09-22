fn test_ownership() {
    println!("=====ownership=====");
    // 固定大小，栈上分配
    let s = "hello";
    // 栈上数据，直接拷贝, 没有交换所有权
    let s_copy = s;
    // s 和 s_copy 均可访问
    println!("{:?}", (s, s_copy));

    // 不固定大小，堆上分配
    let mut ss = String::from("hello");
    ss.push_str(", world!");
    // 堆上数据，转移所有权
    let ss_copy = ss;
    // 所有权转移，ss不可访问，只有ss_copy可访问
    // println!("{}", ss); // 编译时错误：Borrow of moved value
    println!("{}", ss_copy);
    // ss_copy离开作用域时自动调用drop函数，释放内存
}

fn test_deep_clone() {
    println!("=====deep clone=====");
    let s1 = String::from("deep clone");

    // rust中任何对堆上数据的"赋值"操作都是所有权转移
    // 如果需要深拷贝，需要使用clone方法(与Clone trait不同)
    let s2 = s1.clone();
    println!("{:?}", (s1, s2));

    // rust中的基本类型(都在栈上分配)都实现了Clone trait，不会发生所有权转移，深拷贝（其实谈不上深拷贝或浅拷贝）
    let _a = 1;
    let _b = _a;
    // 这里 _a _b 都可以访问
}

fn test_assign_in_function() {
    // 本示例演示函数参数的传递方式
    println!("=====parameter in function=====");
    let s = String::from("hello");

    // 堆上数据传递给函数，会转移所有权
    takes_ownership(s);
    // 这里s已经不可访问
    // println!("{}", s); // -> 编译错误：Borrow of moved value

    // 栈上数据传递给函数不会有所有权转移问题
    let ii = 1;
    makes_copy(ii);
    // 这里 ii 仍可访问
    println!("ii: {}", ii);
}

fn takes_ownership(some_string: String) {
    // 这个函数会取走所有权
    println!("{}", some_string);

    // 在这里数据占用的内存会被释放
}

fn makes_copy(some_integer: i32) {
    // 基本数据类型，直接拷贝，不会有所有权转移
    println!("some_integer: {}", some_integer);
}

fn test_borrow() {
    // 传递参数会发生所有权转移在有时候会很麻烦
    // 可以使用借用的概念，在用完后把所有权还回去
    println!("=====borrow=====");

    let s = String::from("hello");
    // 传递引用，不会转移所有权
    borrow_but_not_take(&s);
    // 由于没有所有权转移，此时仍然可以访问
    println!("所有权没有转移:{}", s);

    // 借用的变量不可以更改，如果需要修改，则需要可变引用
    // 当然，变量也需要是可以改变的
    let mut s_mutable = String::from("hello");
    borrow_and_change(&mut s_mutable);
    // 所有权不会转移，但是值已经改变了
    println!("改变后的值：{}", s_mutable);

    // 可变引用只能同时(这里的同时指的是同一个作用域)存在一个
    // 这意味着rust中不会有竞态条件, (从而在多线程下不需要数据同步机制, 是吗？)

    // let r1 = &mut s_mutable;
    // let r2 = &mut s_mutable; // -> 编译错误： 同时借用

    // 不可变引用可以同时有多个
    let rr1 = &s_mutable;
    let rr2 = &s_mutable;
    println!("多个不可变引用:{:?}", (rr1, rr2));

    // 不可变引用和可变引用的作用域不能重叠
    let rr1 = &s_mutable;
    let rr2 = &s_mutable;

    // let p1 = &mut s_mutable;
    // println!("{:?}", (p1, rr1, rr2)); // -> 编译错误 p1 和 （rr1, rr2)的引用交叉了

    println!("不可变引用：{:?}", (rr1, rr2)); // 在这里访问后 rr1 rr2 的作用域就消失了 (编译器行为, NLL)
                                              // 从这里就可以使用可变引用了
    let p1 = &mut s_mutable;
    println!("可变引用：{}", *p1);
}

fn borrow_but_not_take(some_string: &String) {
    println!("借用函数中的值: {}", *some_string);
}

fn borrow_and_change(some_string: &mut String) {
    some_string.push_str(", world!");
    println!("在函数中修改可变引用：{}", some_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership() {
        test_ownership();
        test_deep_clone();
        test_assign_in_function();
        test_borrow();
    }
}
