use std::io::{stdin, stdout};
use std::io::Write;
use alu::*;

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
