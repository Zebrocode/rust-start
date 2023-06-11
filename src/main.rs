#[derive(Debug)]
struct Point <T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

impl  Point<i32> {
    fn x1(&self) -> &i32{
        &self.x
    }
}

#[derive(Debug)]
struct ComplexPoint <T,U> {
    x: T,
    y: U,
}

impl <T,U> ComplexPoint<T,U> {
    fn mixup<V,W>(self, point: ComplexPoint<V,W> ) -> ComplexPoint<T,W> {
        ComplexPoint {
            x: self.x,
            y: point.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T,E>{
    Ok(T),
    Err(E),
}


fn main() { 
    let integer = Point {
        x: 5,
        y: 10,
    };
    let float = Point {
        x: 5.0,
        y: 6.0
    };

    let integerFloat = ComplexPoint {
        x: 5,
        y: 10.0,
    };
    let integerChar = ComplexPoint {
        x: 1,
        y: 'c',
    };
    print!("{:#?} {:#?} {:#?}",integer,float,integerFloat);
    let mix = integerFloat.mixup(integerChar);
    print!("{:#?}", mix);
}

