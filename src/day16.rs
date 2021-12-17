struct BitAccess<'a>(&'a [usize]);

impl BitAccess<'_> {
    const CHUNK_SIZE_BYTES: usize = std::mem::size_of::<usize>();
    const CHUNK_SIZE_BITS: usize = Self::CHUNK_SIZE_BYTES * 8;

    fn new<'a>(data: &'a [usize]) -> BitAccess {
        BitAccess(data)
    }

    fn len(&self) -> usize {
        return self.0.len() * Self::CHUNK_SIZE_BITS;
    }

    fn get(&self, off_bits: usize, num_bits: usize) -> usize {
        const CHUNK_SIZE: usize = BitAccess::CHUNK_SIZE_BITS;
        assert!(num_bits <= CHUNK_SIZE);
        let mut result = 0;
        let mut to_read = num_bits;
        let mut off_in_result = num_bits;
        let mut current_chunk = off_bits / CHUNK_SIZE;
        let mut offset_in_chunk = off_bits % CHUNK_SIZE;
        while to_read > 0 {
            assert!(current_chunk < self.0.len());
            let avail_in_chunk = CHUNK_SIZE - offset_in_chunk;
            let to_read_from_chunk = std::cmp::min(avail_in_chunk, to_read);
            let chunk_val = self.0[current_chunk];
            // [          chunk_size      ]
            // [off_in_chunk][to_read][...]
            let shift_left = offset_in_chunk;
            let shift_right = CHUNK_SIZE - to_read_from_chunk;
            let read_val = (chunk_val << shift_left) >> shift_right;

            // [          chunk_size           ]
            // [...][        num_bits          ]
            // [...][from_chunk_0][from_chunk_1]
            result = result | (read_val << (off_in_result - to_read_from_chunk));
            off_in_result -= to_read_from_chunk;
            to_read -= to_read_from_chunk;
            offset_in_chunk += to_read_from_chunk;
            if offset_in_chunk == CHUNK_SIZE {
                offset_in_chunk = 0;
                current_chunk += 1;
            }
            assert!(offset_in_chunk < CHUNK_SIZE);
        }
        assert_eq!(to_read, 0);
        assert_eq!(off_in_result, 0);
        result
    }
}

struct BitReader<'a> {
    bits: BitAccess<'a>,
    offset: usize,
}

impl BitReader<'_> {
    fn new<'a>(data: &'a [usize]) -> BitReader<'a> {
        BitReader {
            bits: BitAccess::new(data),
            offset: 0,
        }
    }

    fn avail(&self) -> usize {
        self.bits.len() - self.offset
    }

    fn read(&mut self, num_bits: usize) -> usize {
        let result = self.bits.get(self.offset, num_bits);
        self.offset += num_bits;
        assert!(self.offset <= self.bits.len());
        result
    }
}

struct PacketReader<'a>(BitReader<'a>);

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum PacketData {
    Literal(usize),
    Operator { op: Op, sub_packets: Vec<Packet> },
}

#[derive(Debug)]
struct Packet {
    version: usize,
    data: PacketData,
}

enum SubPacketsLength {
    Bits(usize),
    Packets(usize),
}

impl PacketReader<'_> {
    fn new<'a>(data: &'a [usize]) -> PacketReader {
        PacketReader(BitReader::new(data))
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.0.read(3);
        let typ = self.0.read(3);
        let data = match typ {
            4 => PacketData::Literal(self.read_literal()),
            _ => PacketData::Operator {
                op: Self::op_from_typ(typ),
                sub_packets: self.read_sub_packets(),
            },
        };
        Packet { version, data }
    }

    fn op_from_typ(typ: usize) -> Op {
        match typ {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Minimum,
            3 => Op::Maximum,
            5 => Op::GreaterThan,
            6 => Op::LessThan,
            7 => Op::EqualTo,
            _ => panic!("unexpected packet type {}", typ),
        }
    }

    fn read_literal(&mut self) -> usize {
        let mut val = 0;
        loop {
            let is_last = self.0.read(1) == 0;
            let bits = self.0.read(4);
            val = (val << 4) | bits;
            if is_last {
                break;
            }
        }
        val
    }

    fn read_sub_packets(&mut self) -> Vec<Packet> {
        let mut packets = Vec::new();
        let len = self.read_length();
        match len {
            SubPacketsLength::Bits(len) => {
                let expected_avail = self.0.avail() - len;
                while self.0.avail() > expected_avail {
                    packets.push(self.read_packet());
                }
                assert_eq!(self.0.avail(), expected_avail);
            }
            SubPacketsLength::Packets(count) => {
                for _ in 0..count {
                    packets.push(self.read_packet());
                }
            }
        }
        packets
    }

    fn read_length(&mut self) -> SubPacketsLength {
        match self.0.read(1) {
            0 => SubPacketsLength::Bits(self.0.read(15)),
            1 => SubPacketsLength::Packets(self.0.read(11)),
            _ => panic!("unreachable code"),
        }
    }
}

fn parse_string(s: &str) -> Vec<usize> {
    s.as_bytes()
        .chunks(BitAccess::CHUNK_SIZE_BYTES * 2)
        .map(|bytes| std::str::from_utf8(bytes).unwrap())
        .map(|s| {
            usize::from_str_radix(s, 16).unwrap() << (BitAccess::CHUNK_SIZE_BITS - s.len() * 4)
        })
        .collect::<Vec<_>>()
}

fn packet_from_str(s: &str) -> Packet {
    let data = parse_string(s);
    let mut packet_reader = PacketReader::new(&data);
    packet_reader.read_packet()
}

fn sum_versions(packet: &Packet) -> usize {
    let mut sum = packet.version;
    if let PacketData::Operator { op: _, sub_packets } = &packet.data {
        for sub_packet in sub_packets {
            sum += sum_versions(sub_packet);
        }
    }
    sum
}

fn calc_packet_result(packet: &Packet) -> usize {
    match &packet.data {
        PacketData::Literal(v) => *v,
        PacketData::Operator { op, sub_packets } => {
            let mut sub_val_iter = sub_packets.iter().map(|p| calc_packet_result(p));
            match op {
                Op::Sum => sub_val_iter.fold(0, |sum, val| val + sum),
                Op::Product => sub_val_iter.fold(1, |product, val| val * product),
                Op::Minimum => sub_val_iter.min().unwrap(),
                Op::Maximum => sub_val_iter.max().unwrap(),
                _ if sub_packets.len() == 2 => {
                    let v0 = sub_val_iter.next();
                    let v1 = sub_val_iter.next();
                    let result = match op {
                        Op::LessThan => v0 < v1,
                        Op::GreaterThan => v0 > v1,
                        Op::EqualTo => v0 == v1,
                        _ => panic!("bad packet{:?}", packet)
                    };
                    if result { 1 } else { 0 }
                }
                Op::LessThan => sub_val_iter.max().unwrap(),
                _ => panic!("bad packet{:?}", packet)
            }
        }
    }
}

pub fn main() {
    test_bit_access();
    test_bit_reader();
    test_parse_string();
    test_packet_reader();
    test_sum_versions();
    test_calc_packet_result();

    let day16_packet = packet_from_str(&std::fs::read_to_string("input/day16.txt").unwrap());
    let pt1_result = sum_versions(&day16_packet);
    let pt2_result = calc_packet_result(&day16_packet);
    println!("day16 pt1 {}\nday 16 pt2 {}", pt1_result, pt2_result);
}

fn test_calc_packet_result() {
    let tests = [
        ("C200B40A82", 3),     //finds the sum of 1 and 2, resulting in the value 3.
        ("04005AC33890", 54),  //finds the product of 6 and 9, resulting in the value 54.
        ("880086C3E88112", 7), //finds the minimum of 7, 8, and 9, resulting in the value 7.
        ("CE00C43D881120", 9), //finds the maximum of 7, 8, and 9, resulting in the value 9.
        ("D8005AC2A8F0", 1),   //produces 1, because 5 is less than 15.
        ("F600BC2D8F", 0),     //produces 0, because 5 is not greater than 15.
        ("9C005AC2F8F0", 0),   //produces 0, because 5 is not equal to 15.
        ("9C0141080250320F1802104A08", 1), //produces 1, because 1 + 3 = 2 * 2.
    ];

    for test in tests.iter() {
        let packet = packet_from_str(test.0);
        assert_eq!(calc_packet_result(&packet), test.1, "{}", test.0);
    }
}

fn test_sum_versions() {
    let tests = [
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ];

    for test in tests.iter() {
        let packet = packet_from_str(test.0);
        assert_eq!(sum_versions(&packet), test.1);
    }
}

fn test_packet_reader() {
    let data = parse_string("D2FE28");
    let mut packet_reader = PacketReader::new(&data);
    let packet = packet_reader.read_packet();
    assert_eq!(packet.version, 6);
    assert!(matches!(packet.data, PacketData::Literal(2021)));

    let data = parse_string("38006F45291200");
    let mut packet_reader = PacketReader::new(&data);
    let packet = packet_reader.read_packet();
    assert_eq!(
        format!("{:?}", packet),
        format!(
            "{:?}",
            Packet {
                version: 1,
                data: PacketData::Operator {
                    op: Op::LessThan,
                    sub_packets: vec![
                        Packet {
                            version: 6,
                            data: PacketData::Literal(10)
                        },
                        Packet {
                            version: 2,
                            data: PacketData::Literal(20)
                        }
                    ]
                }
            }
        )
    );

    let data = parse_string("EE00D40C823060");
    let mut packet_reader = PacketReader::new(&data);
    let packet = packet_reader.read_packet();
    assert_eq!(
        format!("{:?}", packet),
        format!(
            "{:?}",
            Packet {
                version: 7,
                data: PacketData::Operator {
                    op: Op::Maximum,
                    sub_packets: vec![
                        Packet {
                            version: 2,
                            data: PacketData::Literal(1)
                        },
                        Packet {
                            version: 4,
                            data: PacketData::Literal(2)
                        },
                        Packet {
                            version: 1,
                            data: PacketData::Literal(3)
                        }
                    ]
                }
            }
        )
    );
}

fn test_bit_access() {
    let test = [
        0b1011110000100000000000000000000000000000000000000000000000001011,
        0b1101110000100000000000000000000000000000000000000000000000000101,
    ];
    let bit_access = BitAccess::new(&test);
    assert_eq!(bit_access.get(0, 5), 0b10111);
    assert_eq!(bit_access.get(3, 6), 0b111000);
    assert_eq!(bit_access.get(60, 8), 0b10111101);
    assert_eq!(bit_access.get(64, 4), 0b1101);
    assert_eq!(bit_access.get(65, 4), 0b1011);
    assert_eq!(
        bit_access.get(32, 64),
        0b101111011100001000000000000000000000
    );
    assert_eq!(bit_access.get(60, 4), 0b1011);
    assert_eq!(bit_access.get(128 - 3, 3), 0b101);
    assert_eq!(bit_access.get(128 - 2, 2), 0b1);
}

fn test_bit_reader() {
    let test = [
        0b1011110000100000000000000000000000000000000000000000000000001011,
        0b1101110000100000000000000000000000000000000000000000000000000101,
    ];
    let mut bit_reader = BitReader::new(&test);
    assert_eq!(bit_reader.read(3), 0b101);
    assert_eq!(
        bit_reader.read(61),
        0b1110000100000000000000000000000000000000000000000000000001011
    );
    assert_eq!(bit_reader.read(3), 0b110);
    assert_eq!(
        bit_reader.read(61),
        0b1110000100000000000000000000000000000000000000000000000000101
    );
    let mut bit_reader = BitReader::new(&test);
    assert_eq!(bit_reader.avail(), 128);
    assert_eq!(
        bit_reader.read(61),
        0b1011110000100000000000000000000000000000000000000000000000001
    );
    assert_eq!(bit_reader.avail(), 67);
    assert_eq!(bit_reader.read(6), 0b11110);
    assert_eq!(bit_reader.avail(), 61);
    assert_eq!(
        bit_reader.read(61),
        0b1110000100000000000000000000000000000000000000000000000000101
    );
    assert_eq!(bit_reader.avail(), 0);
}

fn test_parse_string() {
    let data = parse_string("D2FE28");
    let mut reader = BitReader::new(&data);
    assert_eq!(reader.read(3), 0b110);
    assert_eq!(reader.read(3), 0b100);
    assert_eq!(reader.read(5), 0b10111);
    assert_eq!(reader.read(5), 0b11110);
    assert_eq!(reader.read(5), 0b00101);
    assert_eq!(reader.read(3), 0);

    let data = parse_string("DEADBEAF0000BADF00D");
    let mut reader = BitReader::new(&data);
    assert_eq!(reader.read(8 * 4), 0xdeadbeaf);
    assert_eq!(reader.read(4 * 4), 0);
    assert_eq!(reader.read(7 * 4), 0xbadf00d);
}
