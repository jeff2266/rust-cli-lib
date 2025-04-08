mod option;

use option::*;

#[derive(Debug)]
pub struct Command {
    options: Vec<Option>,
}

impl Command {
    pub fn print_usage(&self) {
        println!("usage:");
        println!("Hi! here are options: {:#?}", self.options);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut command = Command {
            options: Vec::new(),
        };
        command.options.push(Option { short_name: 'd' });
        command.print_usage();
        assert_eq!(1, 1);
    }
}
