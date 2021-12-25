use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

mod input;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn left(&self) -> Coordinates {
        Coordinates {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Coordinates {
        Coordinates {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn up(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn down(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Left,
    Down,
    Right,
    Up,
    None,
}

type Risk = u32;

struct PathNote {
    minimal_risk_sum: Risk,
    reached_in: Direction,
}

type RiskField = Vec<Vec<Risk>>;

struct Cave {
    risk_levels: RiskField,
    width: usize,
    height: usize,
    size_factor: usize,
    path_notes: HashMap<Coordinates, PathNote>,
}

impl Cave {
    fn create(risk_levels: RiskField) -> Cave {
        let width = risk_levels[0].len();
        let height = risk_levels.len();
        Cave {
            risk_levels,
            width,
            height,
            size_factor: 1,
            path_notes: HashMap::new(),
        }
    }

    fn traverse(&mut self) -> Risk {
        // start at 0|0, no risk so far
        let initial_coordinate = Coordinates { x: 0, y: 0 };
        let path_note = PathNote {
            minimal_risk_sum: 0,
            reached_in: Direction::None,
        };
        // reset previous knowledge
        self.path_notes.clear();
        self.path_notes
            .insert(initial_coordinate.clone(), path_note);

        let mut current_coordinates = HashSet::from([initial_coordinate]);

        loop {
            let mut new_coordinates = HashSet::<Coordinates>::new();
            for node in &current_coordinates {
                let node_note = self.path_notes.get(&node).unwrap();

                for dir in [
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                ]
                .iter()
                {
                    // TODO remove next line..
                    if ((node.x == 0) && (*dir == Direction::Left))
                        || ((node.y == 0) && (*dir == Direction::Up))
                    {
                        continue;
                    }
                    if let Some(new_coordinate) = self.check_move_risk(node, dir) {
                        new_coordinates.insert(new_coordinate);
                    }
                }
            }

            if new_coordinates.len() > 0 {
                current_coordinates = new_coordinates;
            } else {
                break;
            }
        }

        self.risk_sum_at(&Coordinates {
            x: (self.width * self.size_factor) - 1,
            y: (self.height * self.size_factor) - 1,
        })
    }

    fn check_move_risk(
        &mut self,
        origin: &Coordinates,
        direction: &Direction,
    ) -> Option<Coordinates> {
        let coordinate = match direction {
            Direction::Up => origin.up(),
            Direction::Right => origin.right(),
            Direction::Down => origin.down(),
            Direction::Left => origin.left(),
            _ => origin.clone(), // TODO that should not happen, panic!
        };

        if self.is_inside(&coordinate) {
            let new_risk_sum = self.risk_sum_at(&origin) + self.risk_at(&coordinate);

            if new_risk_sum < self.risk_sum_at(&coordinate) {
                self.path_notes.insert(
                    coordinate.clone(),
                    PathNote {
                        reached_in: direction.clone(),
                        minimal_risk_sum: new_risk_sum,
                    },
                );
                Some(coordinate)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn risk_at(&self, coordinate: &Coordinates) -> Risk {
        let increase = coordinate.x / self.height + coordinate.y / self.width;
        self.risk_levels[coordinate.x % self.height][coordinate.y % self.width] % 10
    }

    fn risk_sum_at(&self, coordinate: &Coordinates) -> Risk {
        match self.path_notes.get(coordinate) {
            Some(path_note) => path_note.minimal_risk_sum,
            None => Risk::MAX,
        }
    }

    fn is_inside(&self, coordinate: &Coordinates) -> bool {
        (coordinate.x < (self.width * self.size_factor))
            && (coordinate.y < (self.height * self.size_factor))
    }

    fn print_risks(&self) {
        for i in 0..self.height * self.size_factor {
            let line = (0..self.width * self.size_factor)
                .map(|j| self.risk_at(&Coordinates { x: i, y: j }))
                .collect::<Vec<Risk>>();
            println!("{:?}", line);
        }
    }

    fn print_risk_sums(&self) {
        for i in 0..self.height * self.size_factor {
            let line = (0..self.width * self.size_factor)
                .map(|j| self.risk_sum_at(&Coordinates { x: i, y: j }))
                .collect::<Vec<Risk>>();
            println!("{:?}", line);
        }
    }
}

impl FromStr for Cave {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp = Vec::new();
        let mut lines = s.lines();
        while let Some(line) = lines.next() {
            tmp.push(
                line.chars()
                    .filter_map(|c| (c.to_digit(10)))
                    .collect::<Vec<Risk>>(),
            );
        }

        Ok(Cave::create(tmp))
    }
}

fn main() {
    let mut cave = Cave::from_str(input::TASK1).unwrap();

    let mut risk = cave.traverse();
    println!("Part A's sum is {}", risk);

    cave.size_factor = 5;
    cave.print_risks()
    //risk = cave.traverse();
    //println!("Part B's sum is {}", risk);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_CAVE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    fn create_simple_cave() -> Cave {
        let rl = vec![vec![1, 2, 3], vec![2, 2, 4], vec![1, 1, 2]];

        Cave::create(rl)
    }

    #[test]
    fn cave_creation_works() {
        let mut cave = create_simple_cave();
        println!("a");
        cave.print_risks();
        println!("b");
        cave.size_factor = 5;
        cave.print_risks();
        println!("c");
    }

    #[test]
    fn traverse_works() {
        let mut cave = create_simple_cave();

        cave.traverse();
    }

    #[test]
    fn from_str_works() {
        let mut cave = Cave::from_str(&EXAMPLE_CAVE).unwrap();
        assert_eq!(cave.risk_at(&Coordinates { x: 0, y: 0 }), 1);
        assert_eq!(cave.risk_at(&Coordinates { x: 1, y: 2 }), 8);

        cave.print_risks();
        cave.traverse();
        println!("");
        cave.print_risk_sums();
    }
}
