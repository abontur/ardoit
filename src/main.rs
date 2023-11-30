use std::env;
use chrono::{Local, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Result};

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    description: String,
    status: String,
    creation_time: String
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Not enough arguments".into());
    }

    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("test.json")
        .expect("Cannot open a file.");
    let tasks: serde_json::Value = serde_json::from_reader(file).expect("Json is not well formated");
    println!("{}", serde_json::to_string_pretty(&tasks)?);

    match args[1].as_str() {
        "add" => println!("Add tasks option"),
        "view" => print!("View tasks option"),
            _ => println!("not implemented yet"),
    }

    //let tmp = Task {
    //    id: 2,
    //    description: "Create an todo app".to_owned(),
    //    status: "In-Progress".to_owned(),
    //    creation_time: Local::now().to_string()
    //};

    //let file = File::create("test.json").expect("asda");
    //serde_json::to_writer_pretty(file, &tmp);

    Ok(())
}
