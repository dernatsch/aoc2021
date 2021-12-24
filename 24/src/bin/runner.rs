use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use alu::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let scriptfile = &args[1];

    let file = File::open(scriptfile).unwrap();

    let mut alu = ALU::create(get_input);

    for line in io::BufReader::new(file).lines() {
        println!("run: {}", line.as_ref().unwrap());

        alu.run(&line.unwrap()).unwrap();
    }

    alu.print_state();
}
