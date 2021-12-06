
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Population {
    p: [u64;9],
}

impl FromStr for Population {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p: [u64; 9] = [0; 9];
        for s in s.split(',') {
            let num = s.parse::<usize>().unwrap();
            p[num] += 1;
        }
        Ok(Self{p})
    }
}

impl Population {
    fn advance(&mut self) {
        let old = self.p;
        self.p[0..=7].copy_from_slice(&old[1..=8]);
        self.p[6] += old[0];
        self.p[8] = old[0];
    }

    fn size(&self) -> u64 {
        self.p.iter().sum()
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut pop = data.trim().parse::<Population>().unwrap();

    for _ in 1..=80 {
        pop.advance();
    }

    println!("a) {}", pop.size());

    for _ in 81..=256 {
        pop.advance();
    }

    println!("b) {}", pop.size());
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_population_string: &str = "3,4,3,1,2";

    #[test]
    fn population_works() {
        let mut pop = example_population_string.parse::<Population>().unwrap();

        println!("0 - {:?}", pop);

        for i in 1..=80 {
            pop.advance();
            println!("{} - {:?}", i, pop);
        }

        println!("sum: {}", pop.size());

        assert_eq!(pop.size(), 5934);
    }
}
