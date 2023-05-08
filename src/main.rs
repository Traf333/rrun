#![allow(unused)]
extern crate core;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env::args;
use std::fs;

use clap::Parser;
use serde_json::from_str;

use crate::database::read_data;
use crate::timediff::show_time;

mod timediff;

mod database;
mod poker;

#[derive(Parser, Debug)]
struct Args {
    command: String,
    pattern: Option<String>,
    // number: u32,
    //
    // #[arg(short = 'd', long)]
    // date: String,
}


// #[derive(Deserialize)]
// struct Person {
//     name: String,
//     timezone: u32,
// }
//
// struct Database {
//     persons: [Person]
// }

// fn read_file(pathname: String, n: usize) -> [Person, n] {
//     let file_contents = fs::read_to_string(pathname).expect("Error reading file");
//     let database: Database = from_str(&file_contents).unwrap();
//
//     database.persons
// }

pub fn is_armstrong_number(num: u32) -> bool {
    let sum = num.to_string()
        .chars()
        .fold(0u64, |s, c| s + (c as u32 - 0x30).pow(num.to_string().len() as u32) as u64);
    sum == (num as u64)
}


#[test]
#[ignore]
fn test_ten_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(3_999_999_999));
}

// The following number has an Armstrong sum equal to 2^32 plus itself,
// and therefore will be detected as an Armstrong number if you are
// incorrectly using wrapping arithmetic.
#[test]
#[ignore]
fn test_properly_handles_overflow() {
    assert!(!is_armstrong_number(4_106_098_957));
}

#[test]
fn test_isarm() {
    assert_eq!(is_armstrong_number(153), true)
}

fn main() {
    // let args = Args::parse();
    // let hs = hashmap!("a" => 8);
    println!("hello")
    // match args.command.as_str() {
    //     "timediff" => show_time(args.pattern),
    //     "show_data" => { read_data() }
    //     _ => println!("No command has found with name {}", args.command)
    // }
}
