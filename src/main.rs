#![allow(non_snake_case)]
#![allow(dead_code)]
use std::io;



pub fn main() {
    println!("Hello world\n");
    io::stdin().read_line(&mut String::new()).unwrap();
}