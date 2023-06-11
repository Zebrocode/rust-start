use std::{fs::File, io::{self, Read}};

fn open() -> Result<String, io::Error> {

    let result = File::open("hello.txt");
    let mut f = match result {
        Ok(s)=> s,
        Err(e)=> return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s)  {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn open_simple() -> Result<String, io::Error> {
    let mut buf = String::new();
    File::open("hello.txt")?.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() {
    
    let result = open();

    // unwrap 和expect方法快速处理错误

}



