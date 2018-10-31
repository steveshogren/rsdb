use std::io;
extern crate sqlpop;
use sqlpop::parser::parse_sql;

fn main() {
    println!("Starting sql repl");

    while true {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{}", input);
                let x = parse_sql("Select * from users;").unwrap();
                println!("{:?}", x);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
