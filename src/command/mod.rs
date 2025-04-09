mod option;

use option::*;

#[derive(Debug)]
pub struct Command {
    // options: Vec<Option>,
}

impl Command {
    pub fn print_usage(&self) {
        println!("usage:");
        println!("Hi! here are options: ");
    }
}

