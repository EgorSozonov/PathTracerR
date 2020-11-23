#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::io;

mod vect;
use vect::Vect;
mod output;
mod pathtracer;
extern crate rand;

pub fn main() {
    println!("Hello world\n");
    let v = Vect::new(1.0, 2.0, 3.0);
    println!("{}", v.x);
    io::stdin().read_line(&mut String::new()).unwrap();
    let w = 240;
    let h = 135;
    let samplesCount = 4;
        
    let position = Vect::new(-22, 5, 25);
    let goal = Vect::new(-3, 4, 0).minus(position).normalize();
    pathTrace(w, h, samplesCount, position, goal);
}