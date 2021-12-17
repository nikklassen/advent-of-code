use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day16");
    static ref INPUT: Vec<String> = utils::read_input_lines("day16");
}

#[derive(PartialEq, Debug)]
enum PacketValue {
    Literal(usize),
    SubPackets(Vec<Packet>),
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: usize,
    packet_type: usize,
    val: PacketValue,
}

fn parse_input() -> Vec<u8> {
    parse_hex_string(&INPUT[0])
}

fn parse_hex_string(s: &str) -> Vec<u8> {
    s.chars()
        .flat_map(|d| {
            let mut n = d.to_digit(16).unwrap();
            let mut res = vec![];
            res.push((n & 1) as u8);
            n >>= 1;
            res.push((n & 1) as u8);
            n >>= 1;
            res.push((n & 1) as u8);
            n >>= 1;
            res.push((n & 1) as u8);
            res.reverse();
            res
        })
        .collect()
}

fn bits_to_dec(bits: &[u8]) -> usize {
    let mut val = 0usize;
    for n in bits.iter() {
        val <<= 1;
        val |= (*n) as usize;
    }
    val
}

fn iter_to_bits<I>(bit_iter: I) -> usize
where
    I: std::iter::Iterator<Item = u8>,
{
    let bits = bit_iter.collect::<Vec<_>>();
    bits_to_dec(&bits[..])
}

type ByteStream<'a> = dyn std::iter::Iterator<Item = u8> + 'a;

fn parse_literal(stream: &mut ByteStream) -> PacketValue {
    let mut val = 0usize;
    loop {
        let group_prefix = iter_to_bits(stream.take(1));
        let group = iter_to_bits(stream.take(4));
        val <<= 4;
        val |= group;
        if group_prefix == 0 {
            break;
        }
    }
    PacketValue::Literal(val)
}

fn parse_subpackets(stream: &mut ByteStream) -> PacketValue {
    let length_type_id = iter_to_bits(stream.take(1));
    if length_type_id == 0 {
        let bit_length = iter_to_bits(stream.take(15));
        let mut sub_packet_data = stream.take(bit_length).peekable();
        let mut sub_packets = vec![];
        loop {
            if sub_packet_data.peek().is_none() {
                break;
            }
            let sub_packet = parse_packet(&mut sub_packet_data);
            sub_packets.push(sub_packet);
        }
        PacketValue::SubPackets(sub_packets)
    } else {
        let num_sub_packets = iter_to_bits(stream.take(11));
        let mut sub_packets = vec![];
        for _ in 0..num_sub_packets {
            let sub_packet = parse_packet(stream);
            sub_packets.push(sub_packet);
        }
        PacketValue::SubPackets(sub_packets)
    }
}

fn parse_packet(stream: &mut ByteStream) -> Packet {
    let version = iter_to_bits(stream.take(3));
    let packet_type = iter_to_bits(stream.take(3));
    let val = if packet_type == 4 {
        parse_literal(stream)
    } else {
        parse_subpackets(stream)
    };
    Packet {
        version,
        packet_type,
        val,
    }
}

fn sum_version(p: &Packet) -> usize {
    let mut tot = p.version;
    match p.val {
        PacketValue::Literal(_) => {}
        PacketValue::SubPackets(ref subpackets) => {
            for p in subpackets.iter() {
                tot += sum_version(p);
            }
        }
    }
    tot
}

pub fn part1() -> usize {
    let input = parse_input();
    let packet = parse_packet(&mut input.into_iter());
    sum_version(&packet)
}

fn evaluate_packet(p: &Packet) -> usize {
    let mut res = vec![];
    match p.val {
        PacketValue::Literal(ref n) => {
            return *n;
        }
        PacketValue::SubPackets(ref subpackets) => {
            for p in subpackets.iter() {
                res.push(evaluate_packet(p));
            }
        }
    }
    match p.packet_type {
        0 => res.iter().sum(),
        1 => res.iter().product(),
        2 => *res.iter().min().unwrap(),
        3 => *res.iter().max().unwrap(),
        5 => {
            if res[0] > res[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if res[0] < res[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if res[0] == res[1] {
                1
            } else {
                0
            }
        }
        _ => unreachable!(format!("invalid packet type: {}", p.packet_type)),
    }
}

pub fn part2() -> usize {
    let input = parse_input();
    let packet = parse_packet(&mut input.into_iter());
    evaluate_packet(&packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = parse_hex_string("D2FE28");
        let res = parse_packet(&mut input.iter().cloned());
        assert_eq!(
            res,
            Packet {
                version: 6,
                packet_type: 4,
                val: PacketValue::Literal(2021),
            }
        );
    }

    #[test]
    fn test_parse_subpackets_by_length() {
        let input = parse_hex_string("38006F45291200");
        let res = parse_packet(&mut input.iter().cloned());
        assert_eq!(
            res,
            Packet {
                version: 1,
                packet_type: 6,
                val: PacketValue::SubPackets(vec![
                    Packet {
                        version: 6,
                        packet_type: 4,
                        val: PacketValue::Literal(10),
                    },
                    Packet {
                        version: 2,
                        packet_type: 4,
                        val: PacketValue::Literal(20),
                    },
                ])
            }
        );
    }

    #[test]
    fn test_parse_subpackets_by_count() {
        let input = parse_hex_string("EE00D40C823060");
        let res = parse_packet(&mut input.iter().cloned());
        assert_eq!(
            res,
            Packet {
                version: 7,
                packet_type: 3,
                val: PacketValue::SubPackets(vec![
                    Packet {
                        version: 2,
                        packet_type: 4,
                        val: PacketValue::Literal(1),
                    },
                    Packet {
                        version: 4,
                        packet_type: 4,
                        val: PacketValue::Literal(2),
                    },
                    Packet {
                        version: 1,
                        packet_type: 4,
                        val: PacketValue::Literal(3),
                    },
                ])
            }
        );
    }
}
