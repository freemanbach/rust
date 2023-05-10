
#![allow(unused)]
use std::convert::TryInto;
use std::io;
use std::fs::File;
use std::cmp::Ordering;
// use rand::Rng;
use std::vec::Vec;
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
    io::stdin().read_line(&mut mess).ok().expect("failed to read line");
    println!("Hello, your name is {} ", mess);
}

fn test_string2() -> () {

    let mut mess : String = String::from("Hello Kid,");
    let mut name : String = String::from("David");
    println!("length of two string variables: {}", mess.len() + name.len());
}

fn test_int1() -> i32 {
    return 0;
}

fn test_int2() -> i32 {
    return 0;
}

fn test_float1() -> i32 {
    return 0;
}

fn test_float2() -> i32 {
    return 0;
}

fn main() {
    test_string2();

}
