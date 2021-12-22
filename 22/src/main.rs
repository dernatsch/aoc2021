#[macro_use] extern crate itertools;

use std::ops::{Index, RangeInclusive};
use regex::Regex;


struct Instruction {
    enable: bool,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

impl Instruction {
    fn is_valid(&self, maximum: usize) -> bool {
        self.x.end().clone() < maximum 
            && self.y.end().clone() < maximum
            && self.z.end().clone() < maximum
    }
}

struct ReactorCore<const N: usize> {
    grid: [[[bool; N]; N]; N],
    offset: i64,
}

impl<const N:usize> Index<(usize, usize, usize)> for ReactorCore::<N> {
    type Output = bool;

    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1][index.2]
    }
}

impl<const N: usize> ReactorCore<N> {
    fn parse_step(&self, step_instruction: &str) -> Instruction {
        let re: Regex = Regex::new(r"(on|off)\sx=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

        let cap = re.captures(step_instruction).unwrap();

        let process_num = |input: &str| {
            let num = input.parse::<i64>().unwrap();
            (num + self.offset) as usize
        };

        Instruction {
            enable: &cap[1] == "on",
            x: process_num(&cap[2])..=process_num(&cap[3]),
            y: process_num(&cap[4])..=process_num(&cap[5]),
            z: process_num(&cap[6])..=process_num(&cap[7]),
        }
    }

    fn create(offset: i64) -> ReactorCore<N> {
        ReactorCore::<N> {
            grid: [[[false; N]; N]; N],
            offset: offset,
        }
    }

    fn count_active(&self) -> usize {
        let mut sum: usize = 0;
        for (i, j, k) in iproduct!(0..N, 0..N, 0..N) {
            if self.grid[i][j][k] {
                sum += 1;
            }
        }

        return sum;
    }

    fn do_step(&mut self, step_instruction: &str) {
        let instruction = self.parse_step(step_instruction);

        if !instruction.is_valid(N) {
            return;
        }

        for (i, j, k) in iproduct!(instruction.x, instruction.y, instruction.z) {
            self.grid[i][j][k] = instruction.enable;
        }
    }
}

fn main() {
    let mut core = ReactorCore::<101>::create(50);

    let instructions: [&str; 20] = [
        "on x=-3..43,y=-28..22,z=-6..38",
        "on x=-12..41,y=-24..24,z=3..47",
        "on x=-49..-4,y=-40..10,z=-5..44",
        "on x=-31..19,y=-43..9,z=-24..27",
        "on x=-30..22,y=-2..45,z=-40..9",
        "on x=-35..16,y=-26..26,z=2..49",
        "on x=-25..19,y=-43..9,z=-22..26",
        "on x=-10..37,y=-32..19,z=-10..38",
        "on x=-32..20,y=-9..43,z=-25..23",
        "on x=-40..8,y=-48..-4,z=-14..38",
        "off x=-40..-28,y=-8..1,z=-13..-3",
        "on x=-8..43,y=-7..40,z=-1..44",
        "off x=21..30,y=16..31,z=7..25",
        "on x=-20..30,y=-45..4,z=-2..49",
        "off x=-35..-26,y=-5..13,z=-9..3",
        "on x=-40..9,y=-3..45,z=-13..38",
        "off x=12..27,y=-47..-34,z=6..23",
        "on x=-45..1,y=-41..6,z=-35..17",
        "off x=13..29,y=-27..-12,z=13..31",
        "on x=-6..48,y=-35..19,z=-44..3"
    ];

    for instruction in instructions {
        core.do_step(&instruction);
    }

    println!("{}", core.count_active());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_REBOOT_STEPS: [&str; 4] = [
        "on x=10..12,y=10..12,z=10..12",
        "on x=11..13,y=11..13,z=11..13",
        "off x=9..11,y=9..11,z=9..11",
        "on x=10..10,y=10..10,z=10..10",
    ];

    #[test]
    fn parse_works() {
        let core = ReactorCore::<20>::create(0);
        let instruction = core.parse_step(EXAMPLE_REBOOT_STEPS[0]);

        assert_eq!(instruction.enable, true);
        assert_eq!(instruction.x.start().clone(), 10);
        assert_eq!(instruction.z.end().clone(), 12);
    }

    #[test]
    fn simple_example_works() {

        let mut core = ReactorCore::<20>::create(0);
        
        // nothing should be active in the beginning
        assert_eq!(core.count_active(), 0);
        assert_eq!(core[(0,0,0)], false);

        core.do_step(EXAMPLE_REBOOT_STEPS[0]);
        assert_eq!(core.count_active(), 27);
        assert_eq!(core[(10,10,10)], true);

        core.do_step(EXAMPLE_REBOOT_STEPS[1]);
        assert_eq!(core[(11,11,13)], true);
        assert_eq!(core[(13,13,13)], true);

        core.do_step(EXAMPLE_REBOOT_STEPS[2]);
        core.do_step(EXAMPLE_REBOOT_STEPS[3]);

        assert_eq!(core.count_active(), 39);
    }

    #[test]
    fn advanced_example_works() {
        let instructions: [&str; 22] = [
            "on x=-20..26,y=-36..17,z=-47..7",
            "on x=-20..33,y=-21..23,z=-26..28",
            "on x=-22..28,y=-29..23,z=-38..16",
            "on x=-46..7,y=-6..46,z=-50..-1",
            "on x=-49..1,y=-3..46,z=-24..28",
            "on x=2..47,y=-22..22,z=-23..27",
            "on x=-27..23,y=-28..26,z=-21..29",
            "on x=-39..5,y=-6..47,z=-3..44",
            "on x=-30..21,y=-8..43,z=-13..34",
            "on x=-22..26,y=-27..20,z=-29..19",
            "off x=-48..-32,y=26..41,z=-47..-37",
            "on x=-12..35,y=6..50,z=-50..-2",
            "off x=-48..-32,y=-32..-16,z=-15..-5",
            "on x=-18..26,y=-33..15,z=-7..46",
            "off x=-40..-22,y=-38..-28,z=23..41",
            "on x=-16..35,y=-41..10,z=-47..6",
            "off x=-32..-23,y=11..30,z=-14..3",
            "on x=-49..-5,y=-3..45,z=-29..18",
            "off x=18..30,y=-20..-8,z=-3..13",
            "on x=-41..9,y=-7..43,z=-33..15",
            "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
            "on x=967..23432,y=45373..81175,z=27513..53682"
        ];

        let mut core = ReactorCore::<101>::create(50);

        for instruction in instructions {
            core.do_step(&instruction);
        }

        assert_eq!(core.count_active(), 590784);
    }
}
