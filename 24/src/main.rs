use std::io::{stdin, stdout};
use std::io::Write;

mod alu;

use alu::*;

fn get_input() -> Result<i64, alu::AluError> {

    fn parse(s: &str) -> Result<i64, AluError> {
        match s.trim().parse::<i64>() {
            Ok(i) => Ok(i),
            Err(_) => Err(AluError{err: "parsing of number failed"}),
        }
    }

    let mut s = String::new();

    print!("Enter Number: ");
    stdout().lock().flush().unwrap();

    match stdin().read_line(&mut s) {
        Ok(_) => parse(&s),
        Err(_) => Err(AluError{err: "parsing of input failed"}),
    }
}

fn main() -> Result<(), AluError> {

    fn prompt() -> Option<String> {
        let mut buffer = String::new();
        print!("alu> ");
        stdout().lock().flush().unwrap();

        if stdin().read_line(&mut buffer).is_ok() {
            return Some(buffer);
        } else {
            return None;
        }
    }
    
    let mut alu = ALU::create(get_input);    

    // Interpreter loop
    while let Some(s) = prompt() {
        if let Err(_) = alu.run(&s) {
            println!("error");
        }
        alu.print_state();
    }

    Ok(())    
}
