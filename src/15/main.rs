use std::collections::{BinaryHeap, HashMap};
use std::fmt;

struct Board {
    b: Vec<i32>,
    tile_width: usize,
    tile_height: usize,
    width: usize,
    height: usize,
    big: bool,
}

impl Board {
    fn new(s: &str, big: bool) -> Self {
        let width = s.lines().nth(0).unwrap().len();
        let b: Vec<i32> = s
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c as i32 - '0' as i32)
            .collect();

        let height = b.len() / width;

        if big {
            Self {
                b,
                width: width * 5,
                height: height * 5,
                tile_width: width,
                tile_height: height,
                big,
            }
        } else {
            Self {
                b,
                width,
                height,
                tile_width: width,
                tile_height: height,
                big,
            }
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<i32> {
        if self.big {
            let tilex = x / self.tile_width;
            let x = x % self.tile_width;
            let tiley = y / self.tile_height;
            let y = y % self.tile_height;

            if tilex < 5 && tiley < 5 {
                let risk = self.b[y * self.tile_width + x];
                let inc = tilex as i32 + tiley as i32;

                Some(((risk + inc - 1) % 9) + 1)
            } else {
                None
            }
        } else {
            if x < self.width && y < self.height {
                Some(self.b[y * self.width + x])
            } else {
                None
            }
        }
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .into_iter()
            .filter(|(x, y)| *x < self.width && *y < self.height)
            .collect()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{} x {}]", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get((x, y)).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Node(usize, usize, i32);

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(b: &Board) -> i32 {
    let target = (b.width - 1, b.height - 1);

    let mut dist: HashMap<(usize, usize), i32> = HashMap::new();
    let mut q = BinaryHeap::new();

    for y in 0..b.width {
        for x in 0..b.height {
            dist.insert((x, y), i32::MAX);
        }
    }

    *dist.get_mut(&(0, 0)).unwrap() = 0;

    q.push(Node(0, 0, 0));

    while let Some(Node(x, y, cost)) = q.pop() {
        if (x, y) == target {
            break;
        }

        if cost > dist[&(x, y)] {
            continue;
        }

        for n in b.neighbors((x, y)) {
            let next = Node(n.0, n.1, cost + b.get(n).unwrap());
            if next.2 < dist[&n] {
                *dist.get_mut(&n).unwrap() = next.2;
                q.push(next);
            }
        }
    }

    dist[&target]
}

fn part1() {
    let data = std::fs::read_to_string("src/15/data.txt").unwrap();
    let board = Board::new(&data, false);

    let cost = dijkstra(&board);

    println!("part1: {}", cost);
}

fn part2() {
    let data = std::fs::read_to_string("src/15/data.txt").unwrap();
    let board = Board::new(&data, true);

    let cost = dijkstra(&board);

    println!("part2: {}", cost);
}

fn main() {
    part1();
    part2();
}
