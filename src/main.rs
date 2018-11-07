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
    use sqlpop::ast::Cmd::Stmt;
    use sqlpop::ast::Stmt::Select;


    #[test]
    fn test_select() {
        let x = parse("Select * from users;");
        // println!("{:?}", x);
        let y = x.first();
        if let Some(i) = y {
            if let Some(ref o) = *i {
                if let Stmt (ref stmt) = *o {
                    if let Select (ref sel) = *stmt {
                        println!("{:?}", sel);
                        assert_eq!(1,2);
                    };
                };
                //if let Some(ref o) = *i {
                //    if let Select(ref stru) = *o {
                //        println!("{:?}", stru);
                //        assert_eq!(1,2);
                //    };
                //};
            };
        };
    }
}


