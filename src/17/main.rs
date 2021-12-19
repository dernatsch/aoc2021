use regex::Regex;

//const DATA: &str = "target area: x=20..30, y=-10..-5";
const DATA: &str = "target area: x=155..182, y=-117..-67";

#[derive(Clone, Copy, Debug)]
struct TargetArea {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl TargetArea {
    fn new(s: &str) -> Self {
        let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();

        let x1 = i32::from_str_radix(caps.get(1).unwrap().as_str(), 10).unwrap();
        let x2 = i32::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap();
        let y1 = i32::from_str_radix(caps.get(3).unwrap().as_str(), 10).unwrap();
        let y2 = i32::from_str_radix(caps.get(4).unwrap().as_str(), 10).unwrap();

        TargetArea {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    fn shoot(&self, vel: (i32, i32)) -> Shot {
        Shot::new(*self, vel)
    }

    fn inside(&self, p: (i32, i32)) -> bool {
        p.0 >= self.x && p.0 <= self.x + self.width && p.1 >= self.y && p.1 <= self.y + self.height
    }
}

#[derive(Clone, Copy, Debug)]
struct Shot {
    target: TargetArea,
    position: (i32, i32),
    velocity: (i32, i32),
    target_hit: bool,
}

impl Shot {
    fn new(target: TargetArea, velocity: (i32, i32)) -> Self {
        Self {
            target,
            velocity,
            position: (0, 0),
            target_hit: false,
        }
    }
}

impl Iterator for Shot {
    // x, y, target was hit
    type Item = (i32, i32, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.0 > self.target.x + self.target.width || self.position.1 < self.target.y {
            // probe is to the right or lower than the target and can never hit
            // it
            
            None
        } else {
            let ret = Some((self.position.0, self.position.1, self.target_hit));

            self.position.0 += self.velocity.0;
            self.position.1 += self.velocity.1;

            if self.velocity.0 > 0 {
                self.velocity.0 -= 1;
            } else if self.velocity.0 < 0 {
                self.velocity.0 += 1;
            }

            self.velocity.1 -= 1;

            if self.target.inside(self.position) {
                self.target_hit = true;
            }

            ret
        }
    }
}

fn high_point(s: Shot) -> Option<i32> {
    let states: Vec<(i32, i32, bool)> = s.collect();

    if !states.iter().any(|x| x.2) {
        None
    } else {
        states.into_iter().map(|x| x.1).max()
    }
}

fn part1() {
    let target = TargetArea::new(DATA);

    let mut highest = 0;

    for x in 0..target.x {
        for y in 0..500 {
            if let Some(h) = high_point(target.shoot((x,y))) {
                if h > highest {
                    highest = h;
                }
            }
        }
    }

    println!("part1: {}", highest);
}

fn part2() {
    let target = TargetArea::new(DATA);

    let mut hits = Vec::new();

    for x in 0..=target.x+target.width {
        for y in target.y..500 {
            if let Some(_) = high_point(target.shoot((x,y))) {
                hits.push((x,y));
            }
        }
    }

    println!("part1: {}", hits.len());

}

fn main() {
    part1();
    part2();
}
