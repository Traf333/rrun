#![allow(unused)]
extern crate core;

use std::env::args;
use clap::Parser;

use std::fs;
use serde_json::from_str;

mod timediff;

use crate::timediff::show_time;

mod database;

use crate::database::read_data;

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


fn main() {
    let args = Args::parse();

    match args.command.as_str() {
        "timediff" => show_time(args.pattern),
        "show_data" => { read_data() }
        _ => println!("No command has found with name {}", args.command)
    }
}
