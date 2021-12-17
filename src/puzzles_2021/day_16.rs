use crate::puzzle_input;
extern crate num;

pub fn run() {
    let input: String = puzzle_input::read_string("./input/2021-d16-input.txt");
    let packet = bits_transmission(&input);

    println!("** Part 1 Final: {:?}", packet.sum_versions());
    println!("** Part 2 Final: {:?}", packet.calculate());
}

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
enum LengthTypeId {
    NumBytes = 0,
    NumPackets = 1,
}

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

#[derive(Clone, Debug, PartialEq)]
struct BitsPacket {
    b_version: usize,
    b_type: PacketType,
    b_literal: Option<i64>,
    b_len_type: Option<LengthTypeId>,
    b_len: Option<usize>,
    b_packets: Option<Vec<BitsPacket>>,
}
impl BitsPacket {
    fn sum_versions(&self) -> usize {
        if let Some(packets) = &self.b_packets {
            packets.iter().map(|p| p.sum_versions()).sum::<usize>() + self.b_version
        } else {
            self.b_version
        }
    }

    fn calculate(&self) -> i64 {
        let values: Vec<i64> = if self.b_packets.is_some() {
            self.b_packets
                .as_ref()
                .unwrap()
                .iter()
                .map(|p| p.calculate())
                .collect()
        } else {
            vec![]
        };

        let mut iter = values.iter();

        match self.b_type {
            PacketType::Sum => iter.sum(),
            PacketType::Product => iter.product(),
            PacketType::Minimum => *iter.min().unwrap(),
            PacketType::Maximum => *iter.max().unwrap(),
            PacketType::Literal => self.b_literal.unwrap(),
            PacketType::GreaterThan => (iter.next() > iter.next()) as i64,
            PacketType::LessThan => (iter.next() < iter.next()) as i64,
            PacketType::EqualTo => (iter.next() == iter.next()) as i64,
        }
    }
}

fn read_header(source: &str, i: &mut usize) -> usize {
    let start = *i;
    *i += 3;
    usize::from_str_radix(&source[start..*i], 2).unwrap()
}

fn read_type(source: &str, i: &mut usize) -> PacketType {
    let convert = read_header(source, i);
    num::FromPrimitive::from_usize(convert).unwrap()
}

fn read_bitflag(source: &str, i: &mut usize) -> bool {
    let flag = &source[*i..*i + 1] == "1";
    *i += 1;
    flag
}

fn read_len_type_id(source: &str, i: &mut usize) -> LengthTypeId {
    let bit = &source[*i..*i + 1];
    *i += 1;
    match bit {
        "0" => LengthTypeId::NumBytes,
        _ => LengthTypeId::NumPackets,
    }
}

fn read_literal_group<'a>(source: &'a str, i: &mut usize) -> &'a str {
    let start = *i;
    *i += 4;
    &source[start..*i]
}

fn read_total_length(source: &str, i: &mut usize) -> usize {
    let start = *i;
    *i += 15;
    usize::from_str_radix(&source[start..*i], 2).unwrap()
}

fn read_num_packets(source: &str, i: &mut usize) -> usize {
    let start = *i;
    *i += 11;
    usize::from_str_radix(&source[start..*i], 2).unwrap()
}

// Literal value packets encode a single binary number.
// To do this, the binary number is padded with leading zeroes until its length
// is a multiple of four bits, and then it is broken into groups of four bits.
// Each group is prefixed by a 1 bit except the last group, which is prefixed by a 0 bit.
// These groups of five bits immediately follow the packet header.
fn read_packet_literal(source: &str, i: &mut usize, b_version: usize) -> BitsPacket {
    let mut value = String::new();
    let mut fragment = true;
    let mut chunk: &str;

    while fragment && *i < source.len() - 4 {
        fragment = read_bitflag(source, i);
        chunk = read_literal_group(source, i);
        value.push_str(chunk);
    }

    if let Ok(v) = i64::from_str_radix(&value, 2) {
        BitsPacket {
            b_version,
            b_type: PacketType::Literal,
            b_literal: Some(v),
            b_len_type: None,
            b_len: None,
            b_packets: None,
        }
    } else {
        panic!("Lost our way parsing a literal... ");
    }
}

fn read_num_bytes(source: &str, i: &mut usize, b_version: usize, b_type: PacketType) -> BitsPacket {
    let num_bytes = read_total_length(source, i);
    let packet_start = *i;
    let mut packets = Vec::new();

    while *i - packet_start < num_bytes {
        if let Some(packet) = read_packet(source, i) {
            packets.push(packet);
        } else {
            panic!("Lost our way parsing a nested packet... ");
        }
    }

    BitsPacket {
        b_version,
        b_type,
        b_literal: None,
        b_len_type: Some(LengthTypeId::NumBytes),
        b_len: Some(num_bytes),
        b_packets: Some(packets),
    }
}

fn read_num_subpackets(
    source: &str,
    i: &mut usize,
    b_version: usize,
    b_type: PacketType,
) -> BitsPacket {
    let num_packets = read_num_packets(source, i);
    let mut packets = Vec::with_capacity(num_packets);

    for _ in 0..num_packets {
        if let Some(packet) = read_packet(source, i) {
            packets.push(packet);
        } else {
            panic!("Lost our way parsing a nested packet... ");
        }
    }

    BitsPacket {
        b_version,
        b_type,
        b_literal: None,
        b_len_type: Some(LengthTypeId::NumPackets),
        b_len: Some(num_packets),
        b_packets: Some(packets),
    }
}

fn read_packet(source: &str, i: &mut usize) -> Option<BitsPacket> {
    if *i < source.len() {
        let b_version = read_header(source, i);
        let b_type = read_type(source, i);

        if b_type == PacketType::Literal {
            return Some(read_packet_literal(source, i, b_version));
        } else {
            let b_len_type = read_len_type_id(source, i);
            match b_len_type {
                LengthTypeId::NumBytes => {
                    return Some(read_num_bytes(source, i, b_version, b_type))
                }
                LengthTypeId::NumPackets => {
                    return Some(read_num_subpackets(source, i, b_version, b_type))
                }
            }
        }
    }
    None
}

fn bits_transmission(hex_input: &str) -> BitsPacket {
    let seq = convert_to_binary_from_hex(hex_input);
    let mut i = 0;
    read_packet(&seq, &mut i).unwrap()
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
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
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let packet = bits_transmission("D2FE28");
        assert_eq!(6, packet.b_version);
        assert_eq!(PacketType::Literal, packet.b_type);
        assert_eq!(Some(2021), packet.b_literal);
    }

    #[test]
    fn test_num_bytes() {
        let packet = bits_transmission("38006F45291200");
        assert_eq!(1, packet.b_version);
        assert_eq!(PacketType::LessThan, packet.b_type);
        assert_eq!(None, packet.b_literal);
        assert_eq!(Some(LengthTypeId::NumBytes), packet.b_len_type);
        assert_eq!(Some(27), packet.b_len);

        let packets = packet.b_packets.unwrap();
        assert_eq!(2, packets.len());
        assert_eq!(Some(10), packets[0].b_literal);
        assert_eq!(Some(20), packets[1].b_literal);
    }

    #[test]
    fn test_num_packets() {
        let packet = bits_transmission("EE00D40C823060");
        assert_eq!(7, packet.b_version);
        assert_eq!(PacketType::Maximum, packet.b_type);
        assert_eq!(None, packet.b_literal);
        assert_eq!(Some(LengthTypeId::NumPackets), packet.b_len_type);
        assert_eq!(Some(3), packet.b_len);

        let packets = packet.b_packets.unwrap();
        assert_eq!(3, packets.len());
        assert_eq!(Some(1), packets[0].b_literal);
        assert_eq!(Some(2), packets[1].b_literal);
        assert_eq!(Some(3), packets[2].b_literal);
    }

    #[test]
    fn test_transmission_1() {
        let packet = bits_transmission("8A004A801A8002F478");
        assert_eq!(16, packet.sum_versions());
    }

    #[test]
    fn test_transmission_2() {
        let packet = bits_transmission("620080001611562C8802118E34");
        assert_eq!(12, packet.sum_versions());
    }

    #[test]
    fn test_transmission_3() {
        let packet = bits_transmission("C0015000016115A2E0802F182340");
        assert_eq!(23, packet.sum_versions());
    }

    #[test]
    fn test_transmission_4() {
        let packet = bits_transmission("A0016C880162017C3686B18A3D4780");
        assert_eq!(31, packet.sum_versions());
    }

    #[test]
    fn test_sum() {
        let packet = bits_transmission("C200B40A82");
        println!("{:?}", packet);
        assert_eq!(3, packet.calculate());
    }

    #[test]
    fn test_product() {
        let packet = bits_transmission("04005AC33890");
        println!("{:?}", packet);
        assert_eq!(54, packet.calculate());
    }

    #[test]
    fn test_minimum() {
        let packet = bits_transmission("880086C3E88112");
        println!("{:?}", packet);
        assert_eq!(7, packet.calculate());
    }

    #[test]
    fn test_maximum() {
        let packet = bits_transmission("CE00C43D881120");
        println!("{:?}", packet);
        assert_eq!(9, packet.calculate());
    }

    #[test]
    fn test_lt() {
        let packet = bits_transmission("D8005AC2A8F0");
        println!("{:?}", packet);
        assert_eq!(1, packet.calculate());
    }

    #[test]
    fn test_gt() {
        let packet = bits_transmission("F600BC2D8F");
        println!("{:?}", packet);
        assert_eq!(0, packet.calculate());
    }

    #[test]
    fn test_eq() {
        let packet = bits_transmission("9C005AC2F8F0");
        println!("{:?}", packet);
        assert_eq!(0, packet.calculate());
    }

    #[test]
    fn test_compound() {
        let packet = bits_transmission("9C0141080250320F1802104A08");
        println!("{:?}", packet);
        assert_eq!(1, packet.calculate());
    }
}
