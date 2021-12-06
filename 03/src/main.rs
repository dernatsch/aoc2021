use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;
use std::num::ParseIntError;

const LINELENGTH: usize = 12;

fn main() {
    a();
    b();
}

fn a() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {

        let mut ones_in_column: [u64; LINELENGTH] = [0; LINELENGTH];

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {                
                ones_in_column = update_ones_counter(ones_in_column, ip);
            }
        }

        let gamma = build_gamma(ones_in_column);
        let epsilon = !gamma & 0x0FFF;

        println!("03)\n\ta) array: {:#?}, gamma: {:#b}, {:#b}, res: {}", ones_in_column, gamma, epsilon, gamma*epsilon);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn update_ones_counter(mut ones_in_column: [u64; LINELENGTH], line: String) -> [u64; LINELENGTH] {
    for (i, c) in line.chars().enumerate() {
        if c == '1' {
            ones_in_column[i] += 1;
        }
    }
    
    return ones_in_column;
}

fn build_gamma(ones_in_column: [u64; LINELENGTH]) -> u64 {
    let mut res: u64 = 0;
    
    for (i, d) in ones_in_column.iter().rev().enumerate() {
        let val = match d {
            0..=500 => 0,
            501..=1000 => 1,
            1001_u64..=u64::MAX => 100,
        };
        res |= val << i;
    }

    res
}

fn b(){
    // File hosts must exist in current path before this produces output

    let numbers: Vec<u16> = read_numbers("input.txt").unwrap();

    let oxygen_rating = select_num(&numbers);

    println!("b) oxy: {}", oxygen_rating);
}

fn read_numbers<P>(filename: P) -> Result<Vec<u16>, io::Error>
where P: AsRef<Path>, {

    let lines = read_lines(filename).unwrap();

    let mut v = vec![];
    for line in lines {
        v.push(to_num(&line?));
    }

    Ok(v)
}

fn to_num(line: &str) -> u16{
    let mut num: u16 = 0;
    for (i, c) in line.chars().rev().enumerate() {
        num += match c {
            '0' => 0,
            '1' => 1,
            _ => 0,
        } << i;
    }

    num
}

fn select_num(numbers: &Vec<u16>) -> u16{
    select_num_(numbers, 11)
}

fn select_num_(numbers: &Vec<u16>, index: u8) -> u16{

    if numbers.len() == 1 {
        return numbers[0];
    }

    let mut zero_numbers: Vec<u16> = Vec::new();
    let mut one_numbers: Vec<u16> = Vec::new();

    let mut one_counter = 0;

    for num in numbers {
        let current =  (num >> index) & 0b1;

        if current == 1 {
            one_counter += 1;
            one_numbers.push(*num);
        } else {
            zero_numbers.push(*num);
        }
    }

    let new_index = index - 1;

    if one_counter >= numbers.len()/2 {
        return select_num_(&one_numbers, new_index);
    } else {
        return select_num_(&zero_numbers, new_index);
    }
}
