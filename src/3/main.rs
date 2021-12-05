const LINELEN: usize = 12;

fn part1() {
    let data = std::fs::read_to_string("src/3/data.txt").unwrap();

    let lines: Vec<&str> = data.lines().collect();

    let (gamma, epsilon) = gammaepsilon(&lines);

    println!("part1: {}", gamma as usize * epsilon as usize);
}

/// Computes the number made of the most common bit per
/// bit position and the one with the least common bits.
fn gammaepsilon(data: &Vec<&str>) -> (u16, u16) {
    let mut ones = [0; 16];
    let mut zeroes = [0; 16];

    for l in data {
        for i in 0..LINELEN {
            let c = l.chars().nth(i).expect("unexpected line end");
            if c == '0' {
                zeroes[i] += 1;
            } else {
                ones[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    for i in 0..LINELEN {
        if ones[i] < zeroes[i] {
            gamma <<= 1;
        } else {
            gamma = (gamma << 1) | 1;
        }
    }

    let epsilon = (!gamma) & 0xfff;

    (gamma, epsilon)
}

fn part2() {
    let data = std::fs::read_to_string("src/3/data.txt").unwrap();

    let mut oxy_numbers: Vec<&str> = data.lines().collect();
    let mut co2_numbers = oxy_numbers.clone();

    for pos in 0..LINELEN {
        let (gamma, _epsilon) = gammaepsilon(&oxy_numbers);
        let gamma_bit_set = ((gamma >> (LINELEN-pos-1)) & 1) == 1;

        oxy_numbers = oxy_numbers.into_iter().filter(|x| {
            (x.chars().nth(pos).unwrap() == '1') == gamma_bit_set
        }).collect();

        if oxy_numbers.len() <= 1 {
            break;
        }

    }

    for pos in 0..LINELEN {
        let (_, epsilon) = gammaepsilon(&co2_numbers);
        let epsilon_bit_unset = ((epsilon >> (LINELEN-pos-1)) & 1) == 0;

        co2_numbers = co2_numbers.into_iter().filter(|x| {
            (x.chars().nth(pos).unwrap() == '0') == epsilon_bit_unset
        }).collect();
        
        if co2_numbers.len() <= 1 {
            break;
        }
    }

    let oxy = u32::from_str_radix(oxy_numbers[0], 2).unwrap();
    let co2 = u32::from_str_radix(co2_numbers[0], 2).unwrap();

    println!("part2: {}", oxy * co2);
}

fn main() {
    part1();
    part2();
}
