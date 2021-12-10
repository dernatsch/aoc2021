fn part1() {
    let data = std::fs::read_to_string("src/10/data.txt").unwrap();

    let mut score = 0;
    for l in data.lines() {
        let mut stack = Vec::new();

        for c in l.chars() {
            match c {
                '(' => { stack.push(')'); },
                '[' => { stack.push(']'); },
                '{' => { stack.push('}'); },
                '<' => { stack.push('>'); },
                ')' | ']' | '}' | '>' => {
                    let last = stack.pop().unwrap();
                    if c != last {
                        // syntax error
                        score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };
                        break;
                    }
                },
                _ => {},
            }
        }
    }

    println!("part1: {}", score);
}

fn part2() {
    let data = std::fs::read_to_string("src/10/data.txt").unwrap();

    let mut scores = Vec::new();

    for l in data.lines() {
        let mut stack = Vec::new();
        let mut invalid = false;

        for c in l.chars() {
            match c {
                '(' => { stack.push(')'); },
                '[' => { stack.push(']'); },
                '{' => { stack.push('}'); },
                '<' => { stack.push('>'); },
                ')' | ']' | '}' | '>' => {
                    let last = stack.pop().unwrap();
                    if c != last {
                        invalid = true;
                        break;
                    }
                },
                _ => {},
            }
        }

        if !invalid {
            let mut score: u64 = 0;
            for c in stack.into_iter().rev() {
                score *= 5;
                score += match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(),
                }
            }
            scores.push(score);
        }
    }

    scores.sort_unstable();
    println!("part2: {}", scores[scores.len()/2]);
}

fn main() {
    part1();
    part2();
}
