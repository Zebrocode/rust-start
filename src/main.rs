


fn main() {
    let s: String = String::from("hello world");
    println!("{}", first_word(&s));

    let a: [i32; 5] = [1,2,3,4,5];
    let b: &[i32] = &a[..2];
}

fn first_word(s : &String) -> usize{
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return i;
        }
    }
    return bytes.len();
}

// best practice
fn first_word_slice(s : &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
