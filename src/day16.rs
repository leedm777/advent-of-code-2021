pub trait Packet {
    fn get_version(&self) -> u8;
    fn get_type_id(&self) -> u8;
    fn get_sum_version(&self) -> u32;
    fn get_value(&self) -> u64;
}

pub struct PacketLiteral {
    version: u8,
    type_id: u8, // always 4
    value: u64,
}

impl Packet for PacketLiteral {
    fn get_version(&self) -> u8 {
        self.version
    }
    fn get_type_id(&self) -> u8 {
        self.type_id
    }
    fn get_sum_version(&self) -> u32 {
        self.get_version() as u32
    }
    fn get_value(&self) -> u64 {
        self.value
    }
}

pub struct PacketOperator {
    version: u8,
    type_id: u8,
    sub_packets: Vec<Box<dyn Packet>>,
}

impl Packet for PacketOperator {
    fn get_version(&self) -> u8 {
        self.version
    }
    fn get_type_id(&self) -> u8 {
        self.type_id
    }
    fn get_sum_version(&self) -> u32 {
        let r: u32 = self
            .sub_packets
            .iter()
            .map(|p| p.get_sum_version() as u32)
            .sum();
        r + (self.get_version() as u32)
    }
    fn get_value(&self) -> u64 {
        match self.type_id {
            // 0 -> sum
            0 => self
                .sub_packets
                .iter()
                .fold(0, |sum, p| sum + p.get_value()),
            // 1 -> product
            1 => self
                .sub_packets
                .iter()
                .fold(1, |product, p| product * p.get_value()),
            // 2 -> min
            2 => self
                .sub_packets
                .iter()
                .fold(u64::MAX, |min, p| min.min(p.get_value())),
            // 3 -> max
            3 => self
                .sub_packets
                .iter()
                .fold(0, |max, p| max.max(p.get_value())),
            // 5 -> greater than (1 if sub1 > sub2)
            5 => {
                if self.sub_packets[0].get_value() > self.sub_packets[1].get_value() {
                    1
                } else {
                    0
                }
            }
            // 6 -> less than (1 if sub1 < sub2)
            6 => {
                if self.sub_packets[0].get_value() < self.sub_packets[1].get_value() {
                    1
                } else {
                    0
                }
            }
            // 7 -> equal (1 if sub1 == sub2)
            7 => {
                if self.sub_packets[0].get_value() == self.sub_packets[1].get_value() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid packet type"),
        }
    }
}

struct Bits {
    data: Vec<bool>,
    ptr: usize,
}

impl Bits {
    fn new(input: &str) -> Self {
        let mut data = vec![];
        data.reserve(input.len() * 4);
        for ch in input.trim().chars() {
            let v = ch.to_digit(16).expect("Invalid hex digit");
            for shift in (0..4).rev() {
                data.push((v & (1 << shift)) != 0);
            }
        }

        Self { data, ptr: 0 }
    }

    // Generic-ness from https://github.com/gokberkkocak/adventofcode/blob/master/src/aoc2021/day16.rs
    fn read_bits<T>(&mut self, num: usize) -> T
    where
        T: std::ops::Shl<Output = T> + std::ops::BitOr<Output = T> + From<u8> + Default,
    {
        let mut r = T::default();
        if num > 8 * std::mem::size_of::<T>() {
            panic!("TOO MANY BITS");
        }
        for i in self.ptr..(self.ptr + num) {
            if self.data[i] {
                r = (r << T::from(1)) | T::from(1);
            } else {
                r = r << T::from(1);
            }
        }
        self.ptr += num;

        r
    }

    fn read_packet(&mut self) -> Box<dyn Packet> {
        let version: u8 = self.read_bits(3);
        let type_id: u8 = self.read_bits(3);

        if type_id == 4 {
            let mut value = 0u64;
            let mut last_nibble = false;

            while !last_nibble {
                last_nibble = self.read_bits::<u8>(1) == 0;
                let nibble: u64 = self.read_bits(4);
                value = (value << 4) | nibble;
            }

            return Box::new(PacketLiteral {
                version,
                type_id,
                value,
            });
        }

        // operator
        let length_type_id: u8 = self.read_bits(1);
        let mut sub_packets = vec![];

        if length_type_id == 0 {
            // 15 bits for total length of bits for subpackets
            let num_bits: usize = self.read_bits(15);
            let end_bits = self.ptr + num_bits;

            while self.ptr < end_bits {
                let sub_packet = self.read_packet();
                sub_packets.push(sub_packet);
            }
        } else {
            // 11 bits for the number of subpackets
            let num_packets = self.read_bits(11);

            for _ in 0..num_packets {
                let sub_packet = self.read_packet();
                sub_packets.push(sub_packet);
            }
        }

        Box::new(PacketOperator {
            version,
            type_id,
            sub_packets,
        })
    }
}

pub fn parse(input: &str) -> Box<dyn Packet> {
    Bits::new(input).read_packet()
}

pub fn part1(packet: &Box<dyn Packet>) -> u32 {
    packet.get_sum_version()
}

pub fn part2(packet: &Box<dyn Packet>) -> u64 {
    packet.get_value()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        "8A004A801A8002F478".to_string()
    }

    fn ex2() -> String {
        "620080001611562C8802118E34".to_string()
    }

    fn ex3() -> String {
        "C0015000016115A2E0802F182340".to_string()
    }

    fn ex4() -> String {
        "A0016C880162017C3686B18A3D4780".to_string()
    }

    fn real() -> String {
        util::read_input(16)
    }

    #[test]
    fn test_parse() {
        let input = "0F1E";
        let actual = Bits::new(input);
        assert_eq!(
            actual.data,
            vec![
                false, false, false, false, // 0
                true, true, true, true, // F
                false, false, false, true, // 1
                true, true, true, false, // E
            ]
        );
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 16);
    }
    #[test]
    fn test_part1_ex2() {
        let actual = part1(&parse(&ex2()));
        assert_eq!(actual, 12);
    }
    #[test]
    fn test_part1_ex3() {
        let actual = part1(&parse(&ex3()));
        assert_eq!(actual, 23);
    }
    #[test]
    fn test_part1_ex4() {
        let actual = part1(&parse(&ex4()));
        assert_eq!(actual, 31);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 821);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse("C200B40A82"));
        assert_eq!(actual, 3);
    }
    #[test]
    fn test_part2_ex2() {
        let actual = part2(&parse("04005AC33890"));
        assert_eq!(actual, 54);
    }
    #[test]
    fn test_part2_ex3() {
        let actual = part2(&parse("880086C3E88112"));
        assert_eq!(actual, 7);
    }
    #[test]
    fn test_part2_ex4() {
        let actual = part2(&parse("CE00C43D881120"));
        assert_eq!(actual, 9);
    }
    #[test]
    fn test_part2_ex5() {
        let actual = part2(&parse("D8005AC2A8F0"));
        assert_eq!(actual, 1);
    }
    #[test]
    fn test_part2_ex6() {
        let actual = part2(&parse("F600BC2D8F"));
        assert_eq!(actual, 0);
    }
    #[test]
    fn test_part2_ex7() {
        let actual = part2(&parse("9C005AC2F8F0"));
        assert_eq!(actual, 0);
    }
    #[test]
    fn test_part2_ex8() {
        let actual = part2(&parse("9C0141080250320F1802104A08"));
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 2056021084691);
    }
}
