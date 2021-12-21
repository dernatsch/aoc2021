use std::collections::HashSet;
use std::fmt;

const DATA: &str = include_str!("data.txt");

#[derive(Clone, Copy)]
struct Rules([bool; 512]);

impl Rules {
    fn new(s: &str) -> Self {
        let mut r = [false; 512];

        for (i, c) in s.chars().enumerate() {
            r[i] = c == '#';
        }

        Self(r)
    }
}

impl fmt::Debug for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.0 {
            write!(f, "{}", if x { '#' } else { '.' })?;
        }

        Ok(())
    }
}

#[derive(Clone)]
struct Board {
    b: HashSet<(isize, isize)>,
    topleft: (isize, isize),
    botright: (isize, isize),
    boundary_on: bool,
}

impl Board {
    fn new(s: &str) -> Self {
        let mut r = Self {
            b: HashSet::new(),
            topleft: (0, 0),
            botright: (0, 0),
            boundary_on: false,
        };

        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    let p = (x as isize, y as isize);
                    r.set(p);
                }
            }
        }

        r
    }

    fn neighbor_index(&self, p: (isize, isize)) -> u16 {
        let (x, y) = p;
        let neighbors = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let mut r = 0;
        for i in 0..=8 {
            if self.is_on(neighbors[8 - i]) {
                r |= 1 << i;
            }
        }

        r
    }

    fn set(&mut self, p: (isize, isize)) {
        self.b.insert(p);

        if p.0 < self.topleft.0 {
            self.topleft.0 = p.0;
        }

        if p.1 < self.topleft.1 {
            self.topleft.1 = p.1;
        }

        if p.0 > self.botright.0 {
            self.botright.0 = p.0;
        }

        if p.1 > self.botright.1 {
            self.botright.1 = p.1;
        }
    }

    fn is_on(&self, p: (isize, isize)) -> bool {
        if p.0 >= self.topleft.0
            && p.0 <= self.botright.0
            && p.1 >= self.topleft.1
            && p.1 <= self.botright.1
        {
            self.b.contains(&p)
        } else {
            self.boundary_on
        }
    }

    fn unset(&mut self, p: (isize, isize)) {
        self.b.remove(&p);
    }

    fn step(&mut self, r: Rules) {
        let mut new_self = self.clone();

        let tl = (self.topleft.0 - 1, self.topleft.1 - 1);
        let br = (self.botright.0 + 1, self.botright.1 + 1);

        for y in tl.1..=br.1 {
            for x in tl.0..=br.0 {
                let p = (x, y);
                let i = self.neighbor_index(p);
                assert!(i < 512);

                if r.0[i as usize] {
                    new_self.set(p);
                } else {
                    new_self.unset(p);
                }
            }
        }

        if self.boundary_on {
            new_self.boundary_on = r.0[511];
        } else {
            new_self.boundary_on = r.0[0];
        }

        *self = new_self;
    }

    fn len(&self) -> usize {
        self.b.len()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.topleft.1..=self.botright.1 {
            for x in self.topleft.0..=self.botright.0 {
                let p = (x, y);
                write!(f, "{}", if self.b.contains(&p) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part1() {
    let rules = Rules::new(DATA.lines().nth(0).unwrap());
    let mut board = Board::new(DATA.split_once("\n\n").unwrap().1);

    board.step(rules);
    board.step(rules);
    println!("part1: {}", board.len());
}

fn part2() {
    let rules = Rules::new(DATA.lines().nth(0).unwrap());
    let mut board = Board::new(DATA.split_once("\n\n").unwrap().1);

    for _ in 0..50 {
        board.step(rules);
    }
    println!("part2: {}", board.len());
}

fn main() {
    part1();
    part2();
}
