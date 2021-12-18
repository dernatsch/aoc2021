fn part1() {
    let data = std::fs::read_to_string("src/8/data.txt").unwrap();

    let mut digits = 0;

    for l in data.lines() {
        let (_, output) = l.split_once('|').unwrap();

        for o in output.trim().split_whitespace() {
            if [2, 3, 4, 7].contains(&o.len()) {
                digits += 1;
            }
        }
    }

    println!("part1: {}", digits);
}

fn overlap_len(a: &str, b: &str) -> usize {
    b.chars().filter(|&c| a.contains(c)).count()
}

fn decode(l: usize, o: usize, f: usize) -> i32 {
    match (l, o, f) {
        (2, 2, 2) => 1,
        (5, 1, 2) => 2,
        (5, 2, 3) => 3,
        (4, 2, 4) => 4,
        (5, 1, 3) => 5,
        (6, 1, 3) => 6,
        (3, 2, 2) => 7,
        (7, 2, 4) => 8,
        (6, 2, 4) => 9,
        (6, 2, 3) => 0,
        (_, _, _) => {
            println!("unexpected: {} {} {}", l, o, f);
            panic!();
        }
    }
}

fn part2() {
    let data = std::fs::read_to_string("src/8/data.txt").unwrap();
    let mut s = 0;

    for l in data.lines() {
        let (pattern, output) = l.split_once('|').unwrap();
        let patterns: Vec<String> = pattern
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        let one = patterns.iter().find(|x| x.len() == 2).unwrap();
        let four = patterns.iter().find(|x| x.len() == 4).unwrap();

        let mut out = 0;
        for digit in output.split_whitespace() {
            let one_ol = overlap_len(one, digit);
            let four_ol = overlap_len(four, digit);
            let n = decode(digit.len(), one_ol, four_ol);
            out = out * 10 + n;
        }
        s += out;
    }

    println!("part2: {}", s);
}

fn main() {
    part1();
    part2();
}
