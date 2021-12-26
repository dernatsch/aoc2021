use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;

mod input;

// Store received transmission with bitwise readoperation and cursor
// (the structure is stateful)
struct Transmission {
    data: Vec<u8>,
    cursor: usize,
}

impl Transmission {
    fn read(&mut self, num_bits: usize) -> Option<Vec<bool>> {
        let mut result: Vec<bool> = Vec::new();
        let start = self.cursor;        

        while self.cursor < ( start + num_bits ) {
            if let Some(byte) = self.data.get( self.cursor / 8 ) {
                let bit_pos = 7 - (self.cursor % 8); // Read starting from the highest bit
                result.push(((byte >> bit_pos) & 0b1) == 0b1);
                self.cursor += 1;
            } else {
                // we did not get the byte we wanted.. stop trying
                break;
            }
        }

        if num_bits != result.len() {
            None
        } else {
            Some(result)
        }
    }

    fn read_num(&mut self, num_bits: usize) -> Option<u64> {
        let arr = self.read(num_bits)?;

        let mut res = 0;
        for (i, b) in arr.iter().rev().enumerate() {
            if *b { res |= 1 << i; }
        }

        Some(res)
    }

    fn read_bit(&mut self) -> Option<bool> {
        Some(self.read(1)?[0])
    }

    fn chars_to_u8(h: char, l: char) -> Option<u8> {
        if let (Some(highbits), Some(lowbits)) = (h.to_digit(16), l.to_digit(16)) {
            Some( ( (highbits << 4) + lowbits) as u8)
        } else {
            None
        }
    }
}

impl FromStr for Transmission {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<u8> = s.chars()
                               .tuples()
                               .filter_map(|t: (char, char)| Transmission::chars_to_u8(t.0, t.1))
                               .collect();

        Ok(Transmission{ data, cursor: 0})
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

#[derive(Debug)]
enum PacketType {
    Literal{ value: u64 },
    Operator{ sub_packets: Vec<Packet>, operation: u8 },
}

#[derive(Debug)]
enum MyError {
    ParseIntError,
    ParsePacketError,
    OperandNumber,
}

impl Packet{

    fn from_str(s: &str) -> Result<Packet, MyError> {
        if let Ok(mut transmission) = Transmission::from_str(s) {   
            if let Some(packet) =Packet::decode(&mut transmission) {
                Ok(packet)
            } else {
                Err(MyError::ParsePacketError)
            }
        } else {
            Err(MyError::ParseIntError)
        }
    }

    fn decode(transmission: &mut Transmission) -> Option<Packet> {
        let version = transmission.read_num(3)? as u8;
        let packet_type = match transmission.read_num(3)? {
            4 => Some(Packet::decode_literal(transmission)?),
            n => Some(Packet::decode_operator(transmission, n as u8)?),
        }?;

        Some(Packet{version, packet_type})
    }

    fn decode_literal(transmission: &mut Transmission) -> Option<PacketType> {
        let mut value = 0;

        // First bit indicates ongoing packet
        while transmission.read_bit().or(Some(false)).unwrap() {
            for b in transmission.read(4)? {
                value = value << 1;
                if b { value |= 1 }
            }
        }
        
        // TODO duplicate, get rid of it..
        for b in transmission.read(4)? {
            value = value << 1;
            if b { value |= 1 }
        }

        Some(PacketType::Literal{ value })
    }

    fn decode_operator(mut transmission: &mut Transmission, operation: u8) -> Option<PacketType> {
        let mut sub_packets = Vec::new();

        // which kind of lnght is given
        if transmission.read_bit()? {
            // number of sub-packets as 11 bit
            let num_sub_packets = transmission.read_num(11)? as usize;

            for _ in 0..num_sub_packets {
                let sub_packet = Packet::decode(&mut transmission)?;
                sub_packets.push(sub_packet);
            }
        } else {
            // number of bits used for sub-packets as 15 bit
            let num_bits = transmission.read_num(15)? as usize;
            let start = transmission.cursor;
            
            while transmission.cursor < start + num_bits {
                let sub_packet = Packet::decode(&mut transmission)?;
                sub_packets.push(sub_packet);
            }
        }

        Some(PacketType::Operator{ sub_packets, operation })
    }

    fn sum_versions(&self) -> u64{
        let mut sum = self.version as u64;
        if let PacketType::Operator{ sub_packets, .. } = &self.packet_type {
            sum += sub_packets.iter().map(|p| p.sum_versions() as u64).sum::<u64>();
        }
        sum
    }

    fn calculate(&self) -> Result<u64, MyError> {
        match &self.packet_type {
            PacketType::Literal{ value } => Ok(value.clone()),
            PacketType::Operator{ sub_packets, operation } => {
                let vec = sub_packets.iter()
                                     .map(|p| p.calculate())
                                     .collect::<Result<Vec<u64>,_>>()?;
                let mut iter = vec.iter();

                match operation {
                    0 => Ok(iter.sum()),
                    1 => Ok(iter.product()),
                    2 => Ok(*iter.min().unwrap()), // wait until cloned to use ok_or().cloned()
                    3 => Ok(*iter.max().unwrap()), // same here
                    5 => Packet::greater(&mut iter), // greater
                    6 => Packet::less(&mut iter), //less
                    7 => Packet::equal(&mut iter), //equal
                    _ => Ok(0),
                }
            },
        }
    }

    fn greater(iter: &mut dyn Iterator<Item = &u64>) -> Result<u64, MyError> {
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            if a > b {
                Ok(1)
            } else {
                Ok(0)
            }
        } else {
            Err(MyError::OperandNumber)
        }
    }

    fn less(mut iter: &mut dyn Iterator<Item = &u64>) -> Result<u64, MyError> {
        let val = Packet::greater(&mut iter)?;

        if val == 1 {
            Ok(0)
        } else {
            Ok(1)
        }
    }

    fn equal(mut iter: &mut dyn Iterator<Item = &u64>) -> Result<u64, MyError> {
        if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
            if a == b {
                Ok(1)
            } else {
                Ok(0)
            }
        } else {
            Err(MyError::OperandNumber)
        }
    }
}

fn main() {
    let packet = Packet::from_str(input::INPUT).unwrap();

    println!("Answer part 1 is {}", packet.sum_versions());

    println!("Answer part 2 is {}", packet.calculate().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmission_works() {
        let test_str = "8A004A801A8002F478";
        let mut transmission = Transmission::from_str(test_str).unwrap();

        // 2 chars = 1 byte
        assert_eq!(transmission.data.len(), test_str.len() / 2 );

        // check some bytes
        assert_eq!(transmission.data[0], 0x8A);
        assert_eq!(transmission.data[1], 0x00);
        assert_eq!(transmission.data[2], 0x4A);

        // check binary representation and cursor
        let bin_repr_0x8a = vec![true, false, false, false, true, false, true, false];
        assert_eq!(transmission.read(8).unwrap(), bin_repr_0x8a);
    }

    #[test]
    fn decode_literal_works() {
        let test_str = "D2FE28";
        let mut transmission = Transmission::from_str(test_str).unwrap();

        let packet = Packet::decode(&mut transmission).unwrap();

        assert_eq!( packet.version, 6 );

        match packet.packet_type {
            PacketType::Literal{ value } => { assert_eq!(value, 2021)},
            _ => panic!()
        }
    }

    #[test]
    fn decode_operator_works1() {
        let test_str = "38006F45291200";
        let mut transmission = Transmission::from_str(test_str).unwrap();

        let packet = Packet::decode(&mut transmission).unwrap();

        assert_eq!(packet.version, 1);

        match packet.packet_type {
            PacketType::Operator{ sub_packets, .. } => {
                assert_eq!(sub_packets.len(), 2);
                if let PacketType::Literal{ value } = sub_packets[0].packet_type {
                    assert_eq!(value, 10)
                } else { panic!() }

                if let PacketType::Literal{ value } = sub_packets[1].packet_type {
                    assert_eq!(value, 20)
                } else { panic!() }
            },
            _ => {panic!()},
        }
    }

    #[test]
    fn decode_operator_works2() {
        let test_str = "EE00D40C823060";
        let mut transmission = Transmission::from_str(test_str).unwrap();

        let packet = Packet::decode(&mut transmission).unwrap();

        assert_eq!(packet.version, 7);

        match packet.packet_type {
            PacketType::Operator{ sub_packets, ..} => {
                assert_eq!(sub_packets.len(), 3);
                if let PacketType::Literal{ value } = sub_packets[0].packet_type {
                    assert_eq!(value, 1)
                } else { panic!() }

                if let PacketType::Literal{ value } = sub_packets[1].packet_type {
                    assert_eq!(value, 2)
                } else { panic!() }

                if let PacketType::Literal{ value } = sub_packets[2].packet_type {
                    assert_eq!(value, 3)
                } else { panic!() }
            },
            _ => {panic!()},
        }
    }

    #[test]
    fn sum_version_numers_works() {
        let examples = [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for example in examples {
            let packet = Packet::from_str(example.0).unwrap();

            assert_eq!(packet.sum_versions(), example.1);

            println!("{:?}", example);
        }
    }

    #[test]
    fn calculate_works() {
        let examples = [
            ("C200B40A82", 3),      // sum
            ("04005AC33890", 54),   // product
            ("880086C3E88112", 7),  // minimum
            ("CE00C43D881120", 9),  // maximum
            ("D8005AC2A8F0", 1),    // less than
            ("F600BC2D8F", 0),      // not greater
            ("9C005AC2F8F0", 0),    // not equal
            ("9C0141080250320F1802104A08", 1), // equal
        ];

        for example in examples {
            let packet = Packet::from_str(example.0).unwrap();

            assert_eq!(packet.calculate().unwrap(), example.1);

            println!("{:?}", example);
        }
    }
}
