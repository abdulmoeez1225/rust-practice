#![allow(unused)]

use std::io;
// use std::io::*;
// use rand::Rng;
// extern crate rand;

fn main() {
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello,{}! {}", name.trim_end(), greeting);
}
