#[macro_use]
extern crate seq;

use std::collections::HashMap;

fn main() {
    let v: Vec<u8> = seq![1, 2, 3];

    let m: HashMap<char, String> = seq!{
      'a' => "apple".to_string(),
      'b' => "banana".to_string(),
      'c' => "coconut".to_string(),
    };

    println!("{:?}", v);
    println!("{:?}", m);
}
