

fn largest_number(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    return max;
}


fn main() {
    let v = vec![1,2,3];

    println!("{}",largest_number(&v));

}

