use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number");

    let x = 5;
    let y = x;
    println!("{}", x);

    let s = String::from("something");
    let s1 = s;
    println!("{}", s1);

    // println!("{}", s) 是错的，因为s已经被转换了所有权
    // 整数可以这么做因为实现了Copy trait,所有实现了Copy trait的类型可以进行栈复制
    // 实现了Drop trait的不能再实现Copy trait 否则编译错误
}
