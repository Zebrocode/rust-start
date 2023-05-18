use std::collections::HashMap;
fn main() {
    let mut map  = HashMap::new();

    map.insert(String::from("xx"), 10);

    // insert 中如果使用的不是引用,所有权就被转移了

    let a:String = "yy".to_string();
    map.insert(a, 20);

    // println!("{}",a);  error  borrow of a moved value a


    let key = "yy".to_string();
    let value = map.get(&key); // get 传入的是引用  返回的是option

    match value {
        Some(v) => println!("{}", v),
        None => println!("no key exist"),
    }

    for (k, v) in &map{
        println!("key {}  vaule {}",k,v);
    }
}

