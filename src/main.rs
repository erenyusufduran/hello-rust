#![warn(unused_variables)]
#![warn(unused_must_use)]
#![warn(dead_code)]

use std::collections::HashMap;
fn main() {
    // // SCALAR VARIABLE TYPES
    // let unsigned: u8 = 10;
    // let signed: i8 = -10;
    // let float: f32 = 1.2;
    // println!(
    //     "Unsigned: {}, Signed: {}, Float: {}",
    //     unsigned, signed, float
    // );
    // let letter = "c";
    // let emoji = "\u{1F600}"; // :D
    // print!("Letter: {}, emoji: {}", letter, emoji);

    // //  ARRAYS
    // let arr: [u8; 3] = [1, 2, 3];
    // let other_arr: [u8; 5] = [100; 5];
    // println!("Index: {}, Length: {}",arr[0], other_arr.len());
    // println!("{:?}",other_arr);

    // //  TUPLES
    // let tuple: (u8,bool,f32) = (5,true,2.1);
    // let tuple2 = (3,5);
    // let (a,b,c) = tuple; // destructruing

    // //  FUNCTIONS
    // println!("{}",is_even(1));

    // // MUTUABILITY
    // let mut num = 5;
    // num = 3;
    // println!("{}", num);

    // // SLICE AND ARRAYS
    // let arr = [0, 1, 2, 3];
    // let slice = &arr[1..3]; // [1, 2] don't know the length
    // borrowing_slice(arr, slice);

    // // STRINGS
    // let str: &str = "hello world";
    // let mut string: String = String::from("Hello World");
    // let slice = &string[..6];
    // slice.len();
    // string.push('1');
    // string.push_str("! Bob");
    // string.replace("Hello", "Bye");
    // println!("{}", string);

    // // IF - FOR - WHILE
    // let n = 3;
    // if n > 0 {
    //     println!("Greater than 0");
    // } else if n < 0 {
    //     println!("Less than 0");
    // } else {
    //     println!("Equals to 0");
    // }
    // for i in 0..6 {
    //     println!("{}", i);
    // }
    // let mut i = 0;
    // while i<4 {
    //     println!("{}", i);
    //     i+=1;
    //     if i ==3 {
    //         println!("Exit");
    //         break;
    //     }
    // }

    // // MATCH
    // let i = 2;
    // match i {
    //     0 => println!("0"),
    //     1 | 2 => println!("1,2"),
    //     3..=4 => println!("3,4"),
    //     _ => println!("default")
    // }

    // // STRUCT
    // let name = String::from("Bird");
    // let bird = Bird { name, attack: 5 };
    // bird.print_name();
    // println!("{} {}", bird.can_fly(), bird.is_anilmal());

    // // ENUMS
    // let a: MyEnum = MyEnum::A;
    // let b: MyEnum = MyEnum::B(5);
    // let c: MyEnum = MyEnum::C { x: 10, y: 20 };
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);
    // if let MyEnum::B(val) = b {
    //     println!("{}", val);
    // }
    // if let MyEnum::C{x,y} = c {
    //     println!("{} {}", x, y);
    // }

    // // VECTORS
    // let mut vec: Vec<i64> = vec![1,2,3,4,5];
    // vec.len();
    // vec[0];
    // vec.push(6);
    // vec.remove(0);
    // println!("{:?}", vec);

    // // HASH MAPS
    // let mut map = HashMap::new();
    // map.insert(0, "Hi");
    // map.insert(1, "Hi2");
    // println!("{:?}", map);
    // match map.get(&0) {
    //     Some(str) => println!("{}", str),
    //     _ => println!("Doesn't exist in map."),
    // }
    // match map.get(&2) {
    //     Some(str) => println!("{}", str),
    //     _ => println!("Doesn't exist in map."),
    // }
    // map.remove(&0);
    // println!("{:?}", map);

    // // OPTIONS
    // let divide1: Option<i32> = divide(4, 2);
    // let divide2: Option<i32> = divide(2, 3);
    // println!("{:?} unwarps to {}", divide1, divide1.unwrap());
    // println!("{:?} unwarps to {}", divide2, divide2.unwrap());

    // // RESULTS
    // let divide_ = divide_for_result(4, 2);
    // println!("{}",divide_.unwrap_or(100));
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // in rust return value doesn't have any semicolumns.
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}

struct Bird {
    name: String,
    attack: u64,
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_anilmal(&self) -> bool {
        false
    }
}
trait Animal {
    // interface
    fn can_fly(&self) -> bool;
    fn is_anilmal(&self) -> bool {
        true
    }
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C { x: i32, y: i32 },
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

enum myError {
    Error1,
}

fn divide_for_result(dividend: i32, divisor: i32) -> Result<i32, myError> {
    if dividend % divisor != 0 {
        Err(myError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}
