struct Bits {
    input: Box<dyn Iterator<Item = char>>,
    byte: u8,
    byte_pos: u8,
    bits_taken: u64,
}

impl Bits {
    fn new(mut input: impl Iterator<Item = char> + 'static) -> Self {
        let first = input.next().unwrap().to_digit(16).unwrap() as u8;
        Bits {
            input: Box::new(input),
            byte: first,
            byte_pos: 0,
            bits_taken: 0,
        }
    }

    fn take_n(&mut self, n: u8) -> Option<u16> {
        assert!(n <= 16);

        let mut v = 0;
        for _ in 0..n {
            match self.next() {
                None => return None,
                Some(bit) => {
                    v = (v << 1) | (bit & 0x1) as u16;
                }
            }
        }

        self.bits_taken += n as u64;
        Some(v)
    }
}

impl Iterator for Bits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.byte_pos {
                4 => match self.input.next() {
                    Some(c) => {
                        self.byte = c.to_digit(16).unwrap() as u8;
                        self.byte_pos = 0;
                    }
                    None => return None,
                },
                i => {
                    let bit: u8 = (self.byte >> (3 - i)) & 0x1;
                    self.byte_pos += 1;
                    return Some(bit);
                }
            }
        }
    }
}

#[derive(Debug)]
enum Packet {
    LiteralValue {
        version: u16,
        type_id: u16,
        number: u64,
    },
    OperatorTotalLengthBits {
        version: u16,
        type_id: u16,
        packets: Vec<Packet>,
    },
    OperatorNumSubPackets {
        version: u16,
        type_id: u16,
        packets: Vec<Packet>,
    },
}

fn parse_packet(bits: &mut Bits) -> Option<Packet> {
    let version = bits.take_n(3)?;
    let type_id = bits.take_n(3)?;

    match type_id {
        4 => {
            let mut number: u64 = 0;
            loop {
                let prefix = bits.take_n(1)?;
                let byte = bits.take_n(4)?;
                number = (number << 4) + (byte & 0xf) as u64;
                if prefix == 0 {
                    break;
                }
            }
            Some(Packet::LiteralValue {
                version,
                type_id,
                number,
            })
        }
        _ => {
            let length_type_id = bits.take_n(1)?;
            match length_type_id {
                0 => {
                    let total_length_bits = bits.take_n(15)?;

                    let mut packets = Vec::new();
                    let initial_pos = bits.bits_taken;
                    loop {
                        packets.push(parse_packet(bits)?);
                        if (bits.bits_taken - initial_pos) == total_length_bits as u64 {
                            break;
                        }
                    }
                    Some(Packet::OperatorTotalLengthBits {
                        version,
                        type_id,
                        packets,
                    })
                }
                1 => {
                    let num_subpackets = bits.take_n(11)?;
                    let packets: Vec<Packet> = (0..num_subpackets)
                        .map(|_| parse_packet(bits).unwrap())
                        .collect();
                    Some(Packet::OperatorNumSubPackets {
                        version,
                        type_id,
                        packets,
                    })
                }
                _ => unreachable!(),
            }
        }
    }
}

// part 1
fn sum_version_numbers(p: &Packet) -> u64 {
    match p {
        &Packet::LiteralValue { version, .. } => version as u64,
        Packet::OperatorNumSubPackets {
            version,
            type_id: _,
            packets,
        }
        | Packet::OperatorTotalLengthBits {
            version,
            type_id: _,
            packets,
        } => {
            let s: u64 = packets.iter().map(sum_version_numbers).sum();
            (*version as u64) + s
        }
    }
}

fn calculate_expression(p: &Packet) -> u64 {
    match p {
        Packet::LiteralValue {
            version: _,
            type_id: _,
            number,
        } => *number,
        Packet::OperatorNumSubPackets {
            version: _,
            type_id,
            packets,
        }
        | Packet::OperatorTotalLengthBits {
            version: _,
            type_id,
            packets,
        } => {
            let mut exprs = packets.iter().map(calculate_expression);
            match type_id {
                0 => exprs.sum(),
                1 => exprs.product(),
                2 => exprs.min().unwrap(),
                3 => exprs.max().unwrap(),
                5 | 6 | 7 => {
                    let first = exprs.next().unwrap();
                    let second = exprs.next().unwrap();
                    match type_id {
                        5 => (first > second).then(|| 1).unwrap_or(0),
                        6 => (first < second).then(|| 1).unwrap_or(0),
                        7 => (first == second).then(|| 1).unwrap_or(0),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut bits = Bits::new(input.chars());

    let p = parse_packet(&mut bits).unwrap();
    println!("{:?}", p);

    println!("{}", sum_version_numbers(&p));
    println!("{}", calculate_expression(&p));
}
