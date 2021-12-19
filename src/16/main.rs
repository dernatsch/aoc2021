use std::fmt::Write;

const DATA: &str = include_str!("data.txt");

fn hex_to_bits(s: &str) -> Option<String> {
    let mut bits = String::new();

    for c in s.chars() {
        let b = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => {
                return None;
            }
        };

        write!(bits, "{}", b).unwrap();
    }

    Some(bits)
}

#[derive(Debug)]
enum PacketContent {
    Empty,
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    typeid: u8,
    content: PacketContent,
}

fn parse_fint(s: &str, size: usize) -> Option<(&str, u16)> {
    assert!(size == 15 || size == 11);

    let (num, rest) = s.split_at(size);

    let mut ret = 0;
    for c in num.chars() {
        ret <<= 1;
        if c == '1' {
            ret |= 1;
        }
    }

    Some((rest, ret))
}

fn parse_three(s: &str) -> Option<(&str, u8)> {
    let (three, rest) = s.split_at(3);
    assert_eq!(three.len(), 3);

    let mut ret = 0;
    if three.chars().nth(0)? == '1' {
        ret += 4;
    }
    if three.chars().nth(1)? == '1' {
        ret += 2;
    }
    if three.chars().nth(2)? == '1' {
        ret += 1;
    }

    Some((rest, ret))
}

fn parse_four(s: &str) -> Option<(&str, u8)> {
    let (three, rest) = s.split_at(4);
    assert_eq!(three.len(), 4);

    let mut ret = 0;
    if three.chars().nth(0)? == '1' {
        ret += 8;
    }
    if three.chars().nth(1)? == '1' {
        ret += 4;
    }
    if three.chars().nth(2)? == '1' {
        ret += 2;
    }
    if three.chars().nth(3)? == '1' {
        ret += 1;
    }

    Some((rest, ret))
}

fn parse_one(s: &str) -> Option<(&str, bool)> {
    let (one, rest) = s.split_at(1);
    assert_eq!(one.len(), 1);

    Some((rest, one.chars().next()? == '1'))
}

fn parse_literal(s: &str) -> Option<(&str, PacketContent)> {
    let mut ss = s;
    let mut ret: u64 = 0;

    loop {
        let (rest, notlast) = parse_one(ss)?;
        let (rest, val) = parse_four(rest)?;

        ret = (ret << 4) | val as u64;

        ss = rest;
        if !notlast {
            break;
        }
    }

    Some((ss, PacketContent::Literal(ret)))
}

fn parse_operator(s: &str) -> Option<(&str, PacketContent)> {
    let (rest, not_bit_len) = parse_one(s)?;
    let is_bit_len = !not_bit_len;
    let (rest, len) = if is_bit_len {
        parse_fint(rest, 15)?
    } else {
        parse_fint(rest, 11)?
    };
    let mut ss = rest;

    let mut subpackets = Vec::new();

    if is_bit_len {
        while rest.len() - ss.len() < len.into() {
            let (rrest, p) = parse_packet(ss)?;
            subpackets.push(p);
            ss = rrest;
        }
    } else {
        for _ in 0..len {
            let (rest, p) = parse_packet(ss)?;
            subpackets.push(p);
            ss = rest;
        }
    }

    Some((ss, PacketContent::Operator(subpackets)))
}

fn parse_packet(s: &str) -> Option<(&str, Packet)> {
    let (rest, version) = parse_three(s)?;
    let (rest, typeid) = parse_three(rest)?;

    if typeid == 4 {
        let (rest, content) = parse_literal(rest)?;
        Some((
            rest,
            Packet {
                version,
                typeid,
                content,
            },
        ))
    } else {
        let (rest, content) = parse_operator(rest)?;
        Some((
            rest,
            Packet {
                version,
                typeid,
                content,
            },
        ))
    }
}

fn version_sum(p: &Packet) -> u32 {
    let mut s = p.version as u32;

    if let PacketContent::Operator(subpackets) = &p.content {
        for sp in subpackets {
            s += version_sum(&sp);
        }
    }

    s
}

fn eval_packet(p: &Packet) -> Option<u64> {
    match &p.content {
        PacketContent::Operator(sps) => {
            match p.typeid {
                0 => Some(sps.iter().flat_map(|sp| eval_packet(sp)).sum()),
                1 => Some(sps.iter().flat_map(|sp| eval_packet(sp)).product()),
                2 => Some(sps.iter().flat_map(|sp| eval_packet(sp)).min()?),
                3 => Some(sps.iter().flat_map(|sp| eval_packet(sp)).max()?),
                5 => {
                    let left = eval_packet(sps.get(0)?)?;
                    let right = eval_packet(sps.get(1)?)?;

                    Some(if left > right {1} else { 0 })
                }
                6 => {
                    let left = eval_packet(sps.get(0)?)?;
                    let right = eval_packet(sps.get(1)?)?;

                    Some(if left < right {1} else { 0 })
                }
                7 => {
                    let left = eval_packet(sps.get(0)?)?;
                    let right = eval_packet(sps.get(1)?)?;

                    Some(if left == right {1} else { 0 })
                }
                _ => None,
            }
        },
        PacketContent::Literal(n) => {
            Some(*n)
        },
        _ => {
            None
        },
    }
}

fn part1() {
    for l in DATA.lines() {
        let bits = hex_to_bits(l).unwrap();
        let (_, packet) = parse_packet(&bits).unwrap();

        println!("part1: {}", version_sum(&packet));
    }
}

fn part2() {
    for l in DATA.lines() {
        let bits = hex_to_bits(l).unwrap();
        let (_, packet) = parse_packet(&bits).unwrap();

        println!("part2: {}", eval_packet(&packet).unwrap());
    }
}

fn main() {
    part1();
    part2();
}
