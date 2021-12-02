fn part1() {
    let data = std::fs::read_to_string("src/2/data.txt").unwrap();

    let mut depth = 0;
    let mut forward = 0;

    for l in data.lines() {
        let parts: Vec<_> = l.split_whitespace().collect();

        let direction = parts[0];
        let length = u32::from_str_radix(parts[1], 10).unwrap();

        match direction {
            "up" => {
                depth -= length;
            }
            "down" => {
                depth += length;
            }
            "forward" => {
                forward += length;
            }
            _ => {}
        }
    }

    println!("part1: {}", depth * forward);
}

fn part2() {
    let data = std::fs::read_to_string("src/2/data.txt").unwrap();

    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;

    for l in data.lines() {
        let parts: Vec<_> = l.split_whitespace().collect();

        let direction = parts[0];
        let length = u32::from_str_radix(parts[1], 10).unwrap();

        match direction {
            "up" => {
                aim -= length;
            }
            "down" => {
                aim += length;
            }
            "forward" => {
                forward += length;
                depth += aim * length;
            }
            _ => {}
        }
    }

    println!("part2: {}", depth * forward);
}

fn main() {
    part1();
    part2();
}
