#![allow(unused)]
extern crate core;

use std::env::args;
use clap::Parser;

use std::fs;
use serde_json::from_str;
use std::num::ParseIntError;

mod timediff;

use crate::timediff::show_time;

mod database;

use crate::database::read_data;

#[derive(Parser, Debug)]
struct Args {
    command: String,
    pattern: String,
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

fn split_and_parse(s: &str) -> Result<(u32, u32), ParseIntError> {
    let parts: Vec<&str> = s.split(':').collect();
    let h = parts[0].parse::<u32>()?;
    let m = parts[1].parse::<u32>()?;
    Ok((h, m))
}


fn main() {
    let args = Args::parse();

    match args.command.as_str() {
        "timediff" => {
            let (hour, minutes) = split_and_parse(args.pattern.as_str()).unwrap();
            show_time(hour, minutes);
        }
        "show_data" => { read_data() }
        _ => println!("No command has found with name {}", args.command)
    }
}
