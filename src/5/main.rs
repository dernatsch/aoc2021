use std::cmp;
use std::fmt;

#[derive(Clone, Copy)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_left_diagonal(&self) -> bool {
        self.x2 - self.x1 == self.y2 - self.y1 && (self.x2 - self.x1 > 0) == (self.y2 - self.y1 > 0)
    }

    fn is_right_diagnoal(&self) -> bool {
        self.x2 - self.x1 == self.y2 - self.y1 && (self.x2 - self.x1 < 0) == (self.y2 - self.y1 > 0)
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        let r = if self.is_horizontal() {
            let low = cmp::min(self.x1, self.x2);
            let high = cmp::max(self.x1, self.x2);
            self.y1 == y && x <= high && x >= low
        } else if self.is_vertical() {
            let low = cmp::min(self.y1, self.y2);
            let high = cmp::max(self.y1, self.y2);
            self.x1 == x && y <= high && y >= low
        } else {
            false
        };

        r
    }

    fn contains_with_diagonal(&self, x: i32, y: i32) -> bool {
        let r = if self.is_horizontal() {
            let low = cmp::min(self.x1, self.x2);
            let high = cmp::max(self.x1, self.x2);
            self.y1 == y && x <= high && x >= low
        } else if self.is_vertical() {
            let low = cmp::min(self.y1, self.y2);
            let high = cmp::max(self.y1, self.y2);
            self.x1 == x && y <= high && y >= low
        } else if self.is_left_diagonal() {
            // Low and high in this context are not the low and high
            // numerical values but "lower" and "higher" on the board.
            // e.g.: on the board:
            // a.
            // .b
            // a is higher than b, even if it has the lower values
            // in it's position.

            let lowx = cmp::max(self.x1, self.x2);
            let lowy = cmp::max(self.y1, self.y2);
            let highx = cmp::min(self.x1, self.x2);
            let highy = cmp::min(self.y1, self.y2);

            x - highx == y - highy && x >= highx && x <= lowx
        } else {
            let lowx = cmp::min(self.x1, self.x2);
            let lowy = cmp::max(self.y1, self.y2);
            let highx = cmp::max(self.x1, self.x2);
            let highy = cmp::min(self.y1, self.y2);

            highx - x == y - highy && x >= lowx && x <= highx
        };

        r
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({},{} -> {},{})", self.x1, self.y1, self.x2, self.y2)
    }
}

impl std::str::FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut points = s.split(" -> ");
        let pa = points.next().unwrap();
        let pb = points.next().unwrap();

        let mut pa = pa.split(',');
        let mut pb = pb.split(',');

        let x1 = pa.next().unwrap().parse::<i32>().unwrap();
        let y1 = pa.next().unwrap().parse::<i32>().unwrap();
        let x2 = pb.next().unwrap().parse::<i32>().unwrap();
        let y2 = pb.next().unwrap().parse::<i32>().unwrap();

        Ok(Self { x1, y1, x2, y2 })
    }
}

fn print_board(b: &Vec<Line>) {
    for y in 0..10 {
        for x in 0..10 {
            let count = b.iter().filter(|v| v.contains_with_diagonal(x, y)).count();

            if count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }
}

fn part1() {
    let data = std::fs::read_to_string("src/5/data.txt").unwrap();

    let vents: Vec<Line> = data.lines().map(|x| x.parse::<Line>().unwrap()).collect();

    let mut overlaps = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            let count = vents.iter().filter(|v| v.contains(x, y)).count();
            if count >= 2 {
                overlaps += 1;
            }
        }
    }

    println!("part1: {}", overlaps);
}

fn part2() {
    let data = std::fs::read_to_string("src/5/data.txt").unwrap();
    let vents: Vec<Line> = data.lines().map(|x| x.parse::<Line>().unwrap()).collect();

    let mut overlaps = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            let count = vents
                .iter()
                .filter(|v| v.contains_with_diagonal(x, y))
                .count();
            if count >= 2 {
                overlaps += 1;
            }
        }
    }

    println!("part2: {}", overlaps);
}

fn main() {
    part1();
    part2();
}
