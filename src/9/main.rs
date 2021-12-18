use std::collections::HashSet;
use std::fmt;

struct Board {
    heights: Vec<u32>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(s: &str) -> Self {
        let width = s.find(|x: char| x.is_whitespace()).unwrap();
        let heights: Vec<u32> = s
            .chars()
            .filter(|x| !x.is_whitespace())
            .map(|x| (x as u8 - '0' as u8) as u32)
            .collect();

        let height: usize = heights.len() / width;

        Self {
            heights,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let off = y * self.width + x;
            Some(self.heights[off])
        }
    }

    fn set(&mut self, x: usize, y: usize, v: u32) {
        let off = y * self.width + x;
        self.heights[off] = v;
    }

    fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for row in self.heights.chunks(self.width) {
            row.iter().for_each(|x| write!(f, "{}", x).unwrap());
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn part1() {
    let data = std::fs::read_to_string("src/9/data.txt").unwrap();

    let board = Board::new(&data);
    let mut s: u64 = 0;

    for y in 0..board.height {
        for x in 0..board.width {
            let height = board.get(x, y).unwrap();
            let neighbors = Board::neighbors(x, y);

            let lowpoint = neighbors
                .iter()
                .flat_map(|&n| board.get(n.0, n.1))
                .all(|h| h > height);

            if lowpoint {
                s += height as u64 + 1;
            }
        }
    }

    println!("part1: {}", s);
}

fn flood(board: &Board, basins: &mut Board, start: (usize, usize), id: u32) {
    let mut queue = vec![start];

    while queue.len() > 0 {
        let current = queue.pop().unwrap();

        if board.get(current.0, current.1).unwrap() == 9 {
            continue;
        }

        basins.set(current.0, current.1, id);

        for n in Board::neighbors(current.0, current.1) {
            match basins.get(n.0, n.1) {
                Some(0) => {
                    queue.push((n.0, n.1));
                }
                _ => {}
            }
        }
    }
}

fn part2() {
    let data = std::fs::read_to_string("src/9/data.txt").unwrap();

    let board = Board::new(&data);

    // Maps locations to basins, 0 means no basin assigned
    let mut basins = Board {
        heights: vec![0; board.width * board.height],
        height: board.height,
        width: board.width,
    };

    let mut next_basin: u32 = 1; // id of next basin

    let mut to_check = Vec::new();
    for y in 0..board.height {
        for x in 0..board.width {
            to_check.push((x, y));
        }
    }

    let mut to_check = to_check.into_iter();

    while let Some(c) =
        to_check.find(|x| basins.get(x.0, x.1) == Some(0) && board.get(x.0, x.1) != Some(9))
    {
        flood(&board, &mut basins, c, next_basin);
        next_basin += 1;
    }

    let mut sizes: Vec<usize> = Vec::new();
    for x in 1..next_basin {
        sizes.push(basins.heights.iter().filter(|&n| *n == x).count());
    }
    sizes.sort();

    println!(
        "part2: {}",
        sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
    );
}

fn main() {
    part1();
    part2();
}
