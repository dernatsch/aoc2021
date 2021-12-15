use std::fmt::{self, Write};
use std::collections::BTreeMap;

struct Rule {
    pattern: String,
    result: char,
}

impl Rule {
    fn new(s: &str) -> Self {
        let parts = s.split_once(" -> ").unwrap();
        let pattern = String::from(parts.0);
        assert!(pattern.len() == 2);
        let result = parts.1.chars().nth(0).unwrap();

        Self{
            pattern,
            result,
        }
    }

    fn expand(&self) -> String {
        format!("{}{}{}",
             self.pattern.chars().nth(0).unwrap(),
             self.result,
             self.pattern.chars().nth(1).unwrap())
    }

    fn expand_right(&self) -> String {
        format!("{}{}",
                self.result,
                self.pattern.chars().nth(1).unwrap())
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} -> {}", self.pattern, self.result)
    }
}

fn part1() {
    let data = std::fs::read_to_string("src/14/data.txt").unwrap();
    let mut lines = data.lines();

    let mut polymer = String::from(lines.next().unwrap());
    lines.next();

    let rules: Vec<Rule> = lines.map(|x| Rule::new(x)).collect();

    for _ in 0..10 {
        let mut new_polymer = String::new();

        for i in 0..polymer.len()-1 {
            let pattern = &polymer[i..][..2];
            let rule = rules.iter().find(|&x| x.pattern == pattern).unwrap();

            if i == 0 {
                write!(new_polymer, "{}", rule.expand()).unwrap();
            } else {
                write!(new_polymer, "{}", rule.expand_right()).unwrap();
            }
        }

        polymer = new_polymer;
    }

    let mut counts = BTreeMap::new();
    for c in polymer.chars() {
        match counts.get_mut(&c) {
            None => { counts.insert(c, 1); },
            Some(x) => { *x += 1; },
        }
    }

    let max = *counts.values().max().unwrap();
    let min = *counts.values().min().unwrap();

    println!("part1: {}", max - min);
}

fn part2() {
    let data = std::fs::read_to_string("src/14/test.txt").unwrap();
    let mut lines = data.lines();

    let mut polymer = String::from(lines.next().unwrap());
    lines.next();

    let rules: Vec<Rule> = lines.map(|x| Rule::new(x)).collect();

    for i in 0..40 {
        println!("{}", i);
        let mut new_polymer = String::new();

        for i in 0..polymer.len()-1 {
            let pattern = &polymer[i..][..2];
            let rule = rules.iter().find(|&x| x.pattern == pattern).unwrap();

            if i == 0 {
                write!(new_polymer, "{}", rule.expand()).unwrap();
            } else {
                write!(new_polymer, "{}", rule.expand_right()).unwrap();
            }
        }

        polymer = new_polymer;
    }

    let mut counts = BTreeMap::new();
    for c in polymer.chars() {
        match counts.get_mut(&c) {
            None => { counts.insert(c, 1); },
            Some(x) => { *x += 1; },
        }
    }

    let max = *counts.values().max().unwrap();
    let min = *counts.values().min().unwrap();

    println!("part2: {}", max - min);
}

fn main() {
    part1();
    part2();
}
