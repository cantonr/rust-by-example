use std::convert::From;

// From example

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }
//
// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }
//
// fn main() {
//     let num = Number::from(30);
//     println!("My number is {:?}", num);
//
//     // For example we can easily convert a str into a String
//
//     let my_str = "hello";
//     let my_string = String::from(my_str);
//     println!("my_string is: {}", my_string);
// }


// Into example

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
