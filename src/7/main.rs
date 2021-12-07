fn part1() {
    let data = std::fs::read_to_string("src/7/data.txt").unwrap();
    let crabs: Vec<_> =
        data.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    let minpos = *crabs.iter().min().unwrap();
    let maxpos = *crabs.iter().max().unwrap();

    let bestfuel: i64 = (minpos..=maxpos).into_iter().map(
        |p| {
            crabs.iter().map(|x| i64::abs(x-p)).sum()
        }).min().unwrap();

    println!("part1: {}", bestfuel);
}

fn part2() {
    let data = std::fs::read_to_string("src/7/data.txt").unwrap();
    let crabs: Vec<_> =
        data.trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();

    let minpos = *crabs.iter().min().unwrap();
    let maxpos = *crabs.iter().max().unwrap();

    let bestfuel: i64 = (minpos..=maxpos).into_iter().map(
        |p| {
            crabs.iter().map(|x| {
                let diff = i64::abs(x-p);

                // Gauss sum
                (diff * diff + diff)/2
            }).sum()
        }).min().unwrap();

    println!("part1: {}", bestfuel);
}

fn main() {
    part1();
    part2();
}
