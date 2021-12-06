#[derive(Clone, Copy, Debug)]
struct Population([u64; 9]);

impl std::str::FromStr for Population {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = [0; 9];
        for n in s.split(',') {
            let num = n.parse::<usize>().unwrap();
            debug_assert!(num < 9);
            p[num] += 1;
        }

        Ok(Self(p))
    }
}

impl Population {
    fn advance(&mut self) {
        let old = &self.0;
        let new = [
            old[1], // 0
            old[2], // 1
            old[3], // 2
            old[4], // 3
            old[5], // 4
            old[6], // 5
            old[7] + old[0], // 6
            old[8], // 7
            old[0], // 8
            ];

        self.0 = new;
    }

    fn size(&self) -> u64 {
        self.0.iter().sum()
    }
}

fn part1() {
    let data = std::fs::read_to_string("src/6/data.txt").unwrap();
    let mut population =
        data.lines().nth(0).unwrap().parse::<Population>().unwrap();

    for _ in 0..80 {
        population.advance();
    }

    println!("part1: {:?}", population.size());
}

fn part2() {
    let data = std::fs::read_to_string("src/6/data.txt").unwrap();
    let mut population =
        data.lines().nth(0).unwrap().parse::<Population>().unwrap();

    for _ in 0..256 {
        population.advance();
    }

    println!("part2: {:?}", population.size());
}

fn main() {
    part1();
    part2();
}
