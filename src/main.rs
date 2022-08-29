#![allow(unused)]

use std::{cmp::Ordering, io};

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

    // let num_1: f32 = 1.111111111111111;
    // println!("f32:{}", num_1 + 3.111111111111111);
    // let num_2: f64 = 1.111111111111111;
    // println!("f32:{}", num_1 + 0.111111111111111);
    // let mut num_3: u32 = 5;

    // let num_4: u32 = 5;
    // println!("5 + 4={}", num_3 + num_4);
    // println!("5 - 4={}", num_3 - num_4);
    // println!("5 * 4={}", num_3 * num_4);
    // println!("5 / 4={}", num_3 / num_4);
    // println!("5 % 4={}", num_3 % num_4);
    // num_3 += 1;

    // --------> Phase five Random
    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random:{}", random_num);

    // -------> phase six If
    // let age = 8;
    // if (age >= 1) && (age <= 18) {
    //     println!("Important Birthday");
    // } else if (age == 21) || (age == 50) {
    //     println!("Important Birthday");
    // } else if age >= 65 {
    //     println!("Important Birthday");
    // } else {
    //     println!("Not an Important Birthday");
    // }

    // ------> phase seven ternary operator
    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 { true } else { false };
    // println!("Can Vote:{}", can_vote);

    // -------> phase Eight "Match"
    // let age2: i32 = 21;
    // match age2 {
    //     1..=18 => println!("Important Birthday"),
    //     21 | 50 => println!("Important Birthday"),
    //     65..=i32::MAX => println!("Important Birthday"),
    //     _ => println!("Not an Important Birthday"),
    // }

    // ------phase Nine Match example 2
    // let my_age: i32 = 18;
    // let voting_age: i32 = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("Can't Vote"),
    //     Ordering::Greater => println!("Can Vote"),
    //     Ordering::Equal => println!("You gained the right to vote"),
    // };

    // -----> phasse ten Loop
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
}
