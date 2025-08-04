use std::fs::File;
use std::io::prelude::*;

impl Student {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let major = lines.next().unwrap().to_string();

        let mut file
    }
}

fn reading_from_file() {
    // open person.txt and print
    let data = Student::from_file("person.txt"); // update this
    println!("name: {}", config.username);
    println!("major: {}", config.major);
}

fn main() {
    reading_from_console();
    reading_from_file();
}

use std::io::{self, Read, Write};

struct Student {
    name: String,
    major: String,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What is your major? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let major = buffer.trim().parse().unwrap();

    let file = File::open("person.txt").unwrap();
    println!("Hi {}, you are a {} major!", person.name, person.age);
}