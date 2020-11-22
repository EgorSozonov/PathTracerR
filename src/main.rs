#![allow(non_snake_case)]
#![allow(dead_code)]
use std::io;

mod vect;
use vect::Vect;
mod output;
mod pathtracer;


pub fn main() {
    println!("Hello world\n");
    let v = Vect::new(1.0, 2.0, 3.0);
    println!("{}", v.x);
    io::stdin().read_line(&mut String::new()).unwrap();

}