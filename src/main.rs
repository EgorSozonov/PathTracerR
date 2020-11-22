#![allow(non_snake_case)]
#![allow(dead_code)]
use std::io;

struct Vec {
    x: f64,
    y: f64,
    z: f64,
}

pub fn main() {
    println!("Hello world\n");
    io::stdin().read_line(&mut String::new()).unwrap();
}