fn part1() {
    let data = std::fs::read_to_string("src/1/data.txt").unwrap();
    let lines = data.lines();

    let mut nums = Vec::new();

    for l in lines {
        let num: u32 = u32::from_str_radix(l, 10).unwrap();
        nums.push(num);
    }

    let mut c = 0;

    for i in 0..nums.len() - 1 {
        if nums[i] < nums[i + 1] {
            c += 1;
        }
    }

    println!("part1: {}", c);
}

fn part2() {
    let data = std::fs::read_to_string("src/1/data.txt").unwrap();
    let lines = data.lines();

    let mut nums = Vec::new();

    for l in lines {
        let num: u32 = u32::from_str_radix(l, 10).unwrap();
        nums.push(num);
    }

    let mut c = 0;

    for i in 0..nums.len() - 3 {
        if nums[i] < nums[i + 3] {
            c += 1;
        }
    }

    println!("part2: {}", c);
}

fn main() {
    part1();
    part2();
}
