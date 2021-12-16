use crate::days::Day;
use crate::input::get_data_from_file;

pub struct Day16 {}

impl Day for Day16 {
    fn part1(&self, input_root: &str) {
        let packets = parse_packets(input_root);
        let mut ver_sum = 0;
        for packet in packets {
            ver_sum += packet.get_version_sum();
        }
        println!("Sum of all versions is {}", ver_sum);
    }
}

fn parse_packets<'a>(input_root: &str) -> Vec<Packet> {
    let bit_vec: Vec<char> = get_data_from_file(input_root, "day16.txt", |s| s).get(0).unwrap().chars().map(to_bit_str).fold(String::new(), |a, b| a + b).chars().collect();
    let mut packets = Vec::new();
    let mut pos = 0;
    while pos < bit_vec.len() - 7 {
        let (packet, bits_consumed) = parse_packet(&bit_vec, pos);
        packets.push(packet.clone());
        pos += bits_consumed;
        if bits_consumed % 4 != 0 {
            pos += 4 - bits_consumed % 4
        }
    }
    return packets;
}

fn parse_packet<'a>(bit_vec: &Vec<char>, start_pos: usize) -> (Packet, usize) {
    let mut pos = start_pos;
    let mut packet = Packet::new(bit_vec[pos..pos + 3].iter().collect::<String>(), bit_vec[pos + 3..pos + 6].iter().collect::<String>());
    pos += 6;
    if packet.type_id == 4 {
        let mut literal_string = String::new();
        while bit_vec[pos] != '0' {
            literal_string.extend(bit_vec[pos + 1..pos + 5].iter());
            pos += 5;
        }
        //last group too
        literal_string.extend(bit_vec[pos + 1..pos + 5].iter());
        pos += 5;
        packet.value = to_int(literal_string.chars().collect::<String>().as_str());
        // we may need to skip a few bits if we're not a 4-bit boundary
        /*if pos % 4 != 0 {
            pos += 4 - pos % 4;
        }*/
    } else {
        let len_bit = bit_vec[pos];
        pos += 1;
        let mut packet_count_len = 15;
        if len_bit == '1' {
            packet_count_len = 11;
        }
        let sub_packet_size = to_int(&bit_vec[pos..pos + packet_count_len].iter().collect::<String>()) as usize;
        pos += packet_count_len;
        if packet_count_len == 11 {
            // interpret sub_packet_size as number of packets
            for _ in 0..sub_packet_size {
                let (sub_packet, bits_consumed) = parse_packet(bit_vec, pos);
                packet.packets.push(sub_packet);
                pos += bits_consumed;
            }
        } else {
            // interpret sub_packet_size as number of bits
            let mut sub_packet_bits_consumed: usize = 0;
            while sub_packet_bits_consumed < sub_packet_size {
                let (sub_packet, bits_consumed) = parse_packet(bit_vec, pos);
                packet.packets.push(sub_packet);
                pos += bits_consumed;
                sub_packet_bits_consumed += bits_consumed;
            }
        }
    }
    return (packet, pos - start_pos);
}

fn to_bit_str<'a>(c: char) -> &'a str {
    return match c {
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
        _ => ""
    };
}

fn to_int(chars: &str) -> i64 {
    i64::from_str_radix(chars, 2).unwrap()
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Packet {
    version: i64,
    type_id: i64,
    value: i64,
    packets: Vec<Packet>,
}

impl Packet {
    pub fn new(ver_str: String, type_str: String) -> Packet {
        Packet {
            version: to_int(&ver_str),
            type_id: to_int(&type_str),
            value: 0,
            packets: Vec::new(),
        }
    }

    pub fn get_version_sum(&self) -> i64 {
        let mut sum = self.version;
        for p in &self.packets {
            sum += p.get_version_sum();
        }
        return sum;
    }
}
