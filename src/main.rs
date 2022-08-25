#![allow(unused)]

use std::{i64::MAX, io};
// use std::io::*;
// use rand::Rng;
// extern crate rand;

fn main() {
    //----> phase one
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    //----> phase two
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't receive input");

    // println!("Hello,{}! {}", name.trim_end(), greeting);

    //----> phase three shadow
    // const ONE_MIL:u128 = 1_000_000;
    // const FLOAT_NUMBER:f64  = 3.4123123;
    // let age = "47";
    // let mut age: u32 = age.trim().parse()
    // .expect("Age wasn't assigned a number");
    // age = age +1;
    // println!("I'm {} and I want ${}",age ,ONE_MIL)

    // --------> Phase four data types
    // println!("Max u32 :{}", u32::MAX);
    // println!("Max u64 :{}", u64::MAX);
    // println!("Max usize :{}", usize::MAX);
    // println!("Max u128 :{}", u128::MAX);
    // println!("Max f32 : {}", f32::MAX);
    // println!("Max f64: {}", f64::MAX);
    // let is_true = true;
    // let my_grade = 'A';

    // --------> Phase five Math

    let num_1: f32 = 1.111111111111111;
    println!("f32:{}", num_1 + 3.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f32:{}", num_1 + 0.111111111111111);
    let mut num_3: u32 = 5;

    let num_4: u32 = 5;
    println!("5 + 4={}", num_3 + num_4);
    println!("5 - 4={}", num_3 - num_4);
    println!("5 * 4={}", num_3 * num_4);
    println!("5 / 4={}", num_3 / num_4);
    println!("5 % 4={}", num_3 % num_4);
    num_3 += 1;
}
