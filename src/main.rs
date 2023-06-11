#[derive(Debug)]
enum Cell{
    Int(u32),
    Float(f64),
    Text(String)

}

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
    // let a: Vec<u32> = Vec::new();
    let mut v = vec![1,2,3];

    v.push(5);

    let cells = vec![Cell::Int(10),Cell::Float(3.0),Cell::Text(String::from("xxx"))];

    println!("{:?}", v);

    print!("{:#?}",cells);

    println!("{}",largest_number(&v));

}

