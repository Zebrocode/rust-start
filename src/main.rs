


fn main() {
    let s = String::from("hello");
    take_reference(&s);

    // 多个可变引用 引用同一个值 错
    // 可变引用和不可变引用 引用同一个值 错
    // 唯一允许的情况是多个不可变引用引用同一个值
    let s1 = &s;
    let s2 = &s;

    take_reference(s1);
    take_reference(s2);

    // 编译器决定不会产生悬垂引用,也就是说引用的作用域内,值一定是有效的,没有被销毁的
}

fn take_reference(s : &String) -> usize{
    // wrong, write s
    // s.push_str("xxx");

    // yes, read s
    s.len()
}
