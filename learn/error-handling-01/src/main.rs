#![allow(unused)]

use std::io::{BufRead, BufReader};

struct Config {
    name: String,
    age: u16,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name:{} age:{}", self.name, self.age)
    }
}

// 0.
fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let lines = BufReader::new(std::fs::File::open(path)?)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let mut iter = lines.into_iter();
    Ok(Config {
        name: iter.next().ok_or("missing name")?,
        age: iter.next().ok_or("missing age")?.parse::<u16>()?,
    })
}

fn main() {
    match load_config("config.txt") {
        Ok(con) => println!("{}", con),
        Err(e) => println!("{}", e),
    }
    match load_config("singleline.txt") {
        Ok(con) => println!("{}", con),
        Err(e) => println!("{}", e),
    }
    match load_config("unknown.txt") {
        Ok(con) => println!("{}", con),
        Err(e) => println!("{}", e),
    }
    println!("program finished");

    let s = String::from("alkdsjf ");
    s = String::from("asdlfjasldjf");
    // let r2 = &mut s;
}
