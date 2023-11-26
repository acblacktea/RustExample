use std::convert::From;
use std::convert::Into;
#[derive(Debug)]
struct Number {
    value: i32,
}

/*
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
    }
}
 */

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
   // let num = Number::from(30);

    let int = 5;
    let num: Number = int.into();
    println!("my number is {:?}", num);
}