
#![allow(unused)]

use std::io;
use rand::Rng;
use std::fs::File;
use std::vec::Vec;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::io::{Write, BufReader, BufRead, ErrorKind};

fn test_bool() -> () {

    let a : bool = true;
    let b : bool = false;
    print!("Bool Value: {} \n", a);
    print!("Bool Value: {} \n", b);
    print!("\n");

}

fn test_string1() -> () {

    let mut mess : String = String::new();
    print!("What is your first name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mess.trim().to_owned()).ok().expect("failed to read line");
    println!("Hello, your name is {} ", mess);
}

fn test_string2() -> () {

    let mut mess : String = String::from("Hello Kid,");
    let mut name : String = String::from("David");
    println!("length of two string variables: {}", mess.len() + name.len());
}

fn test_string3() -> () {

    let mut mess : String = String::new();
    // push() does 1 Char only
    mess.push('h');
    mess.push_str("obbit is");
    mess.push_str(" here.");
    println!("String is Empty ? {} ", mess.is_empty());
    mess.clear();
    println!("String is Empty ? {} ", mess.is_empty());

}

fn test_string4() -> () {
    let mut mess : String = String::from("Cheesecake");
    mess.truncate(6);
    println!("Ans: {}", mess.as_mut_str());

}

fn test_string5() -> () {

    // test this string split to list
    let mess: Vec<&str> = "The power of cheesecake is with me".split(' ').collect();
    for item in mess {
        println!("Ans {}", item);
    }

}

fn test_int1() -> () {
    
    let mut mess1 : String = String::new();
    print!("Enter in a number ? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mess1).ok().expect("failed to read line");
    let num1 : i32 = mess1.trim().parse().unwrap();

    println!();

    let mut mess2 : String = String::new();
    print!("Enter in a number ? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mess2).ok().expect("failed to read line");
    // i8, i16, i32, i64, i128 types + isize
    // u8, u16, u32, u64, u128 types + usize
    let num2 : i32 = mess2.trim().parse().unwrap();
    // let mut num2 : i32 = mess2.parse::<i32>() { Ok(n) => n, Err(_) => { println!("Error !"); 0 },};
    let mut ans : i32 = num1 + num2;
    println!("Our answer is: {}", ans);
}

fn test_float1() -> () {

    let mut mess1 : String = String::new();
    print!("Enter in a decimal number ? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mess1).ok().expect("failed to read line");
    // f32 and f64 types
    let num1 : f32 = mess1.trim().parse().unwrap();

    println!();

    let mut mess2 : String = String::new();
    print!("Enter in a decimal number ? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mess2).ok().expect("failed to read line");
    let num2 : f32 = mess2.trim().parse().unwrap();
    let mut ans : f32 = num1 + num2;
    println!("Our answer is: {}", ans);
}

fn test_ifelse() -> () {

    let mut tmp : String = String::new();
    print!("Enter in a number ? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tmp).ok().expect("failed to read line");
    let num : i32 = tmp.trim().parse().unwrap();

    if num >= 90 {
        println!("A");
    } else if num >= 80 {
        println!("B");
    } else if num >= 70 {
        println!("C");
    } else if num >= 60 {
        println!("D");
    } else {
        println!("F");
    }
}

fn test_loop() -> () {

    let mut a : i32 = 0;
    loop {
        a += 1;

        if a == 5 {
            println!("Nothing !");
            continue;
        }

        if a == 10 {
            println!("OK");
            break;
        }
    }
}

fn test_ternary() -> () {
	let my_age:i32 = 12;
	let vote : bool = if my_age>=21 { true } else { false };
	println!("I can has vote: {}", vote);
}

fn read_item() -> () {
	let mut mess: String = String::new();
	println!("Enter your mess? ");
	io::stdin().read_line(&mut mess).unwrap();
	println!("Mess received: {}", mess);
}

fn gen_lotto() -> () {

    let mut lotto = Vec::new();
    let mut rand_num : i32 = 0;
    for n in 0..=4 {
        rand_num = rand::thread_rng().gen_range(1..=70);
        lotto.push(rand_num);
    }

    for n in 0..1 {
        rand_num = rand::thread_rng().gen_range(1..=25);
        lotto.push(rand_num);
    }

    for i in 0..lotto.len() {
        println!("Value: {}",  lotto[i]);
    }

    // variable must be usize in while loop
    // when printing out the values in vec
    let mut z : usize = 0;
    while z < lotto.len().try_into().unwrap() {
        println!("{}",lotto[z] );
        z += 1;
    }
}


fn main() {
    gen_lotto();
}
