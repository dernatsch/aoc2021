use std::fmt::{self, Write};
use std::collections::{HashMap};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Pair(char, char);

impl Pair {
    fn new(s: &str) -> Self {
        assert!(s.len() == 2);
        Self(
            s.chars().nth(0).unwrap(),
            s.chars().nth(1).unwrap(),
        )
    }
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Rule {
    pattern: Pair,
    result: Vec<(Pair, i64)>,
    addition: char,
}

impl Rule {
    fn new(s: &str) -> Self {
        let parts = s.split_once(" -> ").unwrap();

        let pair = Pair::new(parts.0);
        let middle = parts.1.chars().nth(0).unwrap();

        let mut result = Vec::new();


        let left_pair = Pair(pair.0, middle);
        let right_pair = Pair(middle, pair.1);

        result.push((pair, -1));
        result.push((left_pair, 1));
        result.push((right_pair, 1));

        Self{
            pattern: pair,
            result,
            addition: middle,
        }
    }
}

#[derive(Clone, Debug)]
struct Polymer {
    pairs: HashMap<Pair, i64>,
    counts: HashMap<char, i64>,
}

impl Polymer {
    fn new(s: &str) -> Self {
        let mut pairs = HashMap::new();

        for i in 0..s.len()-1 {
            let pair = Pair::new(&s[i..][..2]);

            match pairs.get_mut(&pair) {
                None => { pairs.insert(pair, 1); },
                Some(x) => { *x += 1 },
            }
        }

        let mut counts = HashMap::new();
        for c in s.chars() {
            match counts.get_mut(&c) {
                None => { counts.insert(c, 1); },
                Some(x) => { *x += 1 },
            }
        }

        Self {
            pairs,
            counts,
        }
    }

    fn apply(&mut self, r: &Rule, m: i64) {
        for (pair, change) in &r.result {
            match self.pairs.get_mut(&pair) {
                None => { self.pairs.insert(*pair, *change * m); },
                Some(count) => { *count += change * m; },
            }

        }

        match self.counts.get_mut(&r.addition) {
            None => { self.counts.insert(r.addition, 1); },
            Some(x) => { *x += m },
        }
    }

    fn len(&self) -> usize {
        self.counts.values().sum::<i64>() as usize
    }

    fn max(&self) -> usize {
        *self.counts.values().max().unwrap() as usize
    }

    fn min(&self) -> usize {
        *self.counts.values().min().unwrap() as usize
    }
}

fn part1() {
    let data = std::fs::read_to_string("src/14/data.txt").unwrap();
    let mut lines = data.lines();

    let polymer = String::from(lines.next().unwrap());
    let mut polymer = Polymer::new(&polymer);
    lines.next();

    let rules: Vec<Rule> = lines.map(|x| Rule::new(x)).collect();
    let mut ruleset: HashMap<Pair, Rule> = HashMap::new();
    for r in rules {
        ruleset.insert(r.pattern, r);
    }

    for _ in 0..10 {
        let mut new_polymer = polymer.clone();

        polymer.pairs.iter().for_each(|(pair, count)| {
            let rule = ruleset.get(&pair).unwrap();

            // apply rule for every occurence of the pair
            new_polymer.apply(rule, *count);
        });

        polymer = new_polymer;
    }

    println!("part1: {}", polymer.max() - polymer.min());
}

fn part2() {
    let data = std::fs::read_to_string("src/14/data.txt").unwrap();
    let mut lines = data.lines();

    let polymer = String::from(lines.next().unwrap());
    let mut polymer = Polymer::new(&polymer);
    lines.next();

    let rules: Vec<Rule> = lines.map(|x| Rule::new(x)).collect();
    let mut ruleset: HashMap<Pair, Rule> = HashMap::new();
    for r in rules {
        ruleset.insert(r.pattern, r);
    }

    for i in 0..40 {
        let mut new_polymer = polymer.clone();

        polymer.pairs.iter().for_each(|(pair, count)| {
            let rule = ruleset.get(&pair).unwrap();

            // apply rule for every occurence of the pair
            new_polymer.apply(rule, *count);
        });

        polymer = new_polymer;
    }

    println!("part2: {}", polymer.max() - polymer.min());
}

fn main() {
    part1();
    part2();
}
