use std::io;
extern crate sqlpop;
use sqlpop::parser::parse_sql;
use sqlpop::ast::Cmd;

fn main() {
    println!("Starting sql repl");
    parse("select * from users;");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                parse(&input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn parse(s:&str) -> Vec<Option<Cmd>> {
    let x =  parse_sql(s);
    match x {
        Ok(stmt) => {
            println!("{:?}", stmt);
            return stmt;
        },
        Err(er) => {
            println!("-------- Malformed sql! ---------");
            println!("{:?}", er);
            panic!("Malformed sql!");
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sqlpop::ast::*;


    #[test]
    fn test_select() {
        let x = parse("Select * from users;");
        println!("{:?}", x);
        let y = x.first();
        if let Some(stmt) = y {
            match stmt {
                Select {with: w,
                        body: bod,
                        order_by: ord,
                        limit: lim} => {assert_eq!(1,2),},
                _ => assert_eq!(1,2),
            };
        };
        assert_eq!(1,2);
    }
}


