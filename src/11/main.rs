use std::fmt;

#[derive(Clone, Copy, Debug)]
struct Cavern {
    octos: [u8; 100],
}

impl Cavern {
    fn new(p: &[u8]) -> Self {
        let mut octos = [0; 100];
        octos.copy_from_slice(p);
        Self { octos }
    }

    fn zero() -> Self {
        Self { octos: [0; 100] }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x < 10 && y < 10 {
            Some(self.octos[y * 10 + x])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, v: u8) {
        if x < 10 && y < 10 {
            self.octos[y * 10 + x] = v;
        }
    }

    fn neighbors(x: usize, y: usize) -> [(usize, usize); 8] {
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
    }

    /// Performs a timestep and returns the number of flashes.
    fn step(&mut self) -> usize {
        let mut flash_map = Cavern::zero();

        for i in 0..100 {
            self.octos[i] += 1;
        }

        let mut new_flashes = true;
        while new_flashes {
            new_flashes = false;
            for y in 0..10 {
                for x in 0..10 {
                    let energy = self.get(x, y).unwrap();
                    if energy > 9 && flash_map.get(x, y) == Some(0) {
                        // only flash if we have enough energy and haven't
                        // flashed in this iteration
                        flash_map.set(x, y, 1);
                        new_flashes = true;

                        for n in Self::neighbors(x, y).into_iter() {
                            if let Some(e) = self.get(n.0, n.1) {
                                self.set(n.0, n.1, e + 1);
                            }
                        }
                    }
                }
            }
        }

        // reset all that have flashed to 0
        for y in 0..10 {
            for x in 0..10 {
                if flash_map.get(x, y) == Some(1) {
                    self.set(x, y, 0);
                }
            }
        }

        flash_map.octos.into_iter().filter(|&x| x == 1).count()
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for y in 0..10 {
            for x in 0..10 {
                let v = self.get(x, y).unwrap();
                if v == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", v)?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn part1() {
    let data = std::fs::read_to_string("src/11/data.txt").unwrap();

    let nums: Vec<u8> = data
        .lines()
        .flat_map(|l| l.chars().map(|c| c as u8 - '0' as u8))
        .collect();
    assert_eq!(nums.len(), 100);

    let mut flashes = 0;

    let mut c = Cavern::new(&nums);
    for _ in 0..100 {
        flashes += c.step();
    }

    println!("part1: {}", flashes);
}

fn part2() {
    let data = std::fs::read_to_string("src/11/data.txt").unwrap();

    let nums: Vec<u8> = data
        .lines()
        .flat_map(|l| l.chars().map(|c| c as u8 - '0' as u8))
        .collect();
    assert_eq!(nums.len(), 100);

    let mut c = Cavern::new(&nums);

    let mut i = 0;
    while c.step() < 100 {
        i += 1;
    }

    println!("part2: {}", i + 1);
}

fn main() {
    part1();
    part2();
}
