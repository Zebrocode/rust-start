

fn largest_number<T: PartialOrd>(list: &[T]) -> &T {
    let mut max: &T = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// 什么是trait bound
// trait bound怎么用来结构体实现上 impl
// 函数中的trait bound语法怎么写
// 最清晰的where字句怎么表示trait bound
//   where 
//       T: Summary + Display,
//       U: Clone + Debug,


fn main() {
    // let a: Vec<u32> = Vec::new();
    let mut v = vec![1,2,3];

    v.push(5);

    let string_list = vec![String::from("al"), String::from("bi")];

    println!("{}",largest_number(&v));
    println!("{}",largest_number(&string_list));

}

