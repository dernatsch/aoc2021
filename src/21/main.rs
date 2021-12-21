use std::collections::HashMap;

struct DetDie(i32);

impl DetDie {
    fn new() -> Self {
        Self(1)
    }

    fn roll(&mut self) -> i32 {
        let r = self.0;
        self.0 = (self.0 % 100) + 1;
        r
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct State {
    p1: i32,
    p2: i32,
    p1score: i32,
    p2score: i32,
}

struct Game {
    mem: HashMap<State, (usize, usize)>,
}

impl Game {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }

    fn play(&mut self, s: State) -> (usize, usize) {
        if let Some(r) = self.mem.get(&s) {
            *r
        } else {
            if s.p2score >= 21 {
                self.mem.insert(s, (0,1));
                return (0,1);
            }

            let mut res = (0,0);

            for (step,count) in [(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1)] {
                let p1 = (s.p1 -1 + step) % 10 + 1;
                let (w2, w1) = self.play(State {
                    p1: s.p2,
                    p2: p1,
                    p1score: s.p2score,
                    p2score: s.p1score + p1
                });

                res.0 += count*w1;
                res.1 += count*w2;
            }

            res
        }
    }
}

fn part1() {
    let mut p1 = 6;
    let mut p2 = 3;

    let mut p1score = 0;
    let mut p2score = 0;

    let mut die = DetDie::new();

    let mut rolls = 0;

    loop {
        let p1step = die.roll() + die.roll() + die.roll();
        rolls += 3;
        p1 = (p1 - 1 + p1step) % 10 + 1;
        p1score += p1;

        if p1score >= 1000 {
            break;
        }

        let p2step = die.roll() + die.roll() + die.roll();
        rolls += 3;
        p2 = (p2 - 1 + p2step) % 10 + 1;
        p2score += p2;

        if p2score >= 1000 {
            break;
        }
    }

    println!(
        "part1: {}",
        rolls * if p1score > p2score { p2score } else { p1score }
    );
}

fn part2() {
    let mut game = Game::new();

    let res = game.play(State {
        p1: 6,
        p2: 3,
        p1score: 0,
        p2score: 0,
    });
    println!("part2: {}", std::cmp::max(res.0, res.1));
}

fn main() {
    part1();
    part2();
}
