use std::fmt;

#[derive(Clone, Copy)]
struct Board([u32; 25]);

impl Board {
    fn rows_won(&self, drawn: &[u32]) -> bool {
        for r in 0..5 {
            let off = r * 5;
            let mut ncontained = 0;

            for n in &self.0[off..][..5] {
                if drawn.contains(n) {
                    ncontained += 1;
                }
            }

            if ncontained == 5 {
                return true;
            }
        }

        false
    }

    fn columns_won(&self, drawn: &[u32]) -> bool {
        for c in 0..5 {
            let off = c;
            let mut ncontained = 0;

            for n in self.0[off..].into_iter().step_by(5) {
                if drawn.contains(n) {
                    ncontained += 1;
                }
            }

            if ncontained == 5 {
                return true;
            }
        }

        false
    }

    fn is_won(&self, drawn: &[u32]) -> bool {
        let rowwon = self.rows_won(drawn);
        let colwon = self.columns_won(drawn);
        rowwon | colwon
    }

    fn score(&self, drawn: &[u32]) -> u32 {
        let mut score = 0;

        for n in &self.0 {
            if !drawn.contains(n) {
                score += n;
            }
        }

        score
    }
}

impl From<&[&str]> for Board {
    fn from(lines: &[&str]) -> Self {
        let mut b = [0; 25];

        for l in 0..5 {
            let nums: Vec<u32> = lines[l].split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            for n in 0..5 {
                let off = l * 5;
                b[off + n] = nums[n];
            }
        }

        Self(b)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                let off = 5 * y;
                write!(f, "{:2} ", self.0[off + x])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn part1() {
    let data = std::fs::read_to_string("src/4/data.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();

    let numbers: Vec<u32> =
        lines[0].split(',').map(|x| x.parse::<u32>().unwrap()).collect();

    let mut boards = Vec::new();

    let mut off = 2;
    while off < lines.len() {
        boards.push(Board::from(&lines[off..][..5]));
        off += 6;
    }

    for l in 0..numbers.len() {
        let drawn = &numbers[..l];

        for b in &boards {
            if b.is_won(drawn) {
                println!("part1: {}", b.score(drawn) * drawn[drawn.len() - 1]);
                return;
            }
        }
    }
}

fn part2() {
    let data = std::fs::read_to_string("src/4/data.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();

    let numbers: Vec<u32> =
        lines[0].split(',').map(|x| x.parse::<u32>().unwrap()).collect();

    let mut boards = Vec::new();

    let mut off = 2;
    while off < lines.len() {
        boards.push(Board::from(&lines[off..][..5]));
        off += 6;
    }

    let mut scores = Vec::new();
    let mut winners = Vec::new();

    for l in 0..numbers.len() {
        let drawn = &numbers[..l];

        for i in 0..boards.len() {
            if winners.contains(&i) {
                continue;
            }

            let b = &boards[i];

            if b.is_won(drawn) {
                let score = b.score(drawn) * drawn[drawn.len() - 1];
                scores.push(score);
                winners.push(i);
            }
        }
    }

    println!("part2: {}", scores[scores.len() - 1]);
}

fn main() {
    part1();
    part2();
}
