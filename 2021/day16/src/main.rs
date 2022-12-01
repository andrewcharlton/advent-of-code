use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("couldn't read file");
    let mut bits = parse_input(&file).into_iter();
    let packet = parse_packet(&mut bits);

    println!("Part one: {}", version_sum(&packet));
    println!("Part two: {}", operator_sum(&packet));
}

struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    packets: Vec<Packet>,
    length: usize,
}

fn version_sum(packet: &Packet) -> usize {
    let mut sum = packet.version;
    for p in &packet.packets {
        sum += version_sum(&p);
    }
    sum
}

fn operator_sum(packet: &Packet) -> usize {
    match packet.type_id {
        0 => packet.packets.iter().map(|p| operator_sum(p)).sum(),
        1 => packet
            .packets
            .iter()
            .map(|p| operator_sum(p))
            .reduce(|acc, x| acc * x)
            .unwrap(),
        2 => packet
            .packets
            .iter()
            .map(|p| operator_sum(p))
            .min()
            .unwrap(),
        3 => packet
            .packets
            .iter()
            .map(|p| operator_sum(p))
            .max()
            .unwrap(),
        4 => packet.value,
        5 => {
            let x = operator_sum(packet.packets.get(0).unwrap());
            let y = operator_sum(packet.packets.get(1).unwrap());
            if x > y {
                1
            } else {
                0
            }
        }
        6 => {
            let x = operator_sum(packet.packets.get(0).unwrap());
            let y = operator_sum(packet.packets.get(1).unwrap());
            if x < y {
                1
            } else {
                0
            }
        }
        7 => {
            let x = operator_sum(packet.packets.get(0).unwrap());
            let y = operator_sum(packet.packets.get(1).unwrap());
            if x == y {
                1
            } else {
                0
            }
        }
        _ => panic!("unknown operator {}", packet.type_id),
    }
}

fn parse_packet<I>(vals: &mut I) -> Packet
where
    I: Iterator<Item = bool>,
{
    let version = read_bits(vals, 3);
    let type_id = read_bits(vals, 3);
    let mut value = 0;
    let mut packets: Vec<Packet> = Vec::new();
    let mut length = 6;

    // Literal value
    if type_id == 4 {
        loop {
            length += 5;
            let should_continue = vals.next().unwrap();
            value = value * 16 + read_bits(vals, 4);
            if !should_continue {
                break;
            }
        }
    } else {
        // Otherwise must be an operator.
        let length_type_id = vals.next().unwrap();
        if length_type_id {
            let num_packets = read_bits(vals, 11);
            length += 12;
            for _ in 0..num_packets {
                let packet = parse_packet(vals);
                length += packet.length;
                packets.push(packet);
            }
        } else {
            let total_length = read_bits(vals, 15);
            length += 16;
            let mut read_length = 0;
            loop {
                let packet = parse_packet(vals);
                read_length += packet.length;
                packets.push(packet);

                if read_length >= total_length {
                    break;
                }
            }
            length += read_length;
        }
    }

    Packet {
        version,
        type_id,
        value,
        packets,
        length,
    }
}

fn read_bits<I>(vals: &mut I, n: usize) -> usize
where
    I: Iterator<Item = bool>,
{
    let mut x = 0;
    for _ in 0..n {
        x = x * 2 + if vals.next().unwrap() { 1 } else { 0 };
    }
    x
}

fn parse_input(input: &str) -> Vec<bool> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '0' => vec![false, false, false, false],
            '1' => vec![false, false, false, true],
            '2' => vec![false, false, true, false],
            '3' => vec![false, false, true, true],
            '4' => vec![false, true, false, false],
            '5' => vec![false, true, false, true],
            '6' => vec![false, true, true, false],
            '7' => vec![false, true, true, true],
            '8' => vec![true, false, false, false],
            '9' => vec![true, false, false, true],
            'A' => vec![true, false, true, false],
            'B' => vec![true, false, true, true],
            'C' => vec![true, true, false, false],
            'D' => vec![true, true, false, true],
            'E' => vec![true, true, true, false],
            'F' => vec![true, true, true, true],
            _ => panic!("unknown input: {}", c),
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_sum_test() {
        assert_eq!(version("8A004A801A8002F478"), 16, "0");
        assert_eq!(version("620080001611562C8802118E34"), 12, "1");
        assert_eq!(version("C0015000016115A2E0802F182340"), 23, "2");
        assert_eq!(version("A0016C880162017C3686B18A3D4780"), 31, "3");
    }

    #[test]
    fn operator_sum_test() {
        assert_eq!(operator("C200B40A82"), 3, "0");
        assert_eq!(operator("04005AC33890"), 54, "1");
        assert_eq!(operator("880086C3E88112"), 7, "2");
        assert_eq!(operator("CE00C43D881120"), 9, "3");
        assert_eq!(operator("D8005AC2A8F0"), 1, "4");
        assert_eq!(operator("F600BC2D8F"), 0, "5");
        assert_eq!(operator("9C005AC2F8F0"), 0, "6");
        assert_eq!(operator("9C0141080250320F1802104A08"), 1, "7");
    }

    fn version(input: &str) -> usize {
        let mut bits = parse_input(input).into_iter();
        let packet = parse_packet(&mut bits);
        version_sum(&packet)
    }

    fn operator(input: &str) -> usize {
        let mut bits = parse_input(input).into_iter();
        let packet = parse_packet(&mut bits);
        operator_sum(&packet)
    }
}
