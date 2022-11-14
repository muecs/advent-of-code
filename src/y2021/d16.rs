//! Day 16: Packet Decoder

struct Packet {
    pub version: u8,
    pub type_id: u8,
    pub value: i64,
    pub size: usize,
    pub packets: Vec<Packet>,
}

impl Packet {
    fn from_binary(s: &str) -> Self {
        let version = u8::from_str_radix(&s[0..=2], 2).unwrap();
        let type_id = u8::from_str_radix(&s[3..=5], 2).unwrap();
        let mut value = 0;
        let size;
        let mut packets = Vec::new();
        if type_id == 4 {
            // literal value, groups of 5 bits
            let mut number = String::new();
            let mut i = 6;
            loop {
                let last = s[i..=i].eq("0");
                number.push_str(&s[i+1..i+5]);
                i += 5;
                if last {
                    break;
                }
            }
            value = i64::from_str_radix(&number, 2).unwrap();
            size = i;
        } else if s[6..=6].eq("0") {
            // operator packet with payload length
            let length = u16::from_str_radix(&s[7..22], 2).unwrap();
            let mut i = 22;
            while i < 22 + usize::from(length) {
                let packet = Packet::from_binary(&s[i..]);
                i += packet.size;
                packets.push(packet);
            }
            size = i;
        } else {
            // operator packet with payload count
            let count = u16::from_str_radix(&s[7..18], 2).unwrap();
            let mut i = 18;
            for _ in 0..count {
                let packet = Packet::from_binary(&s[i..]);
                i += packet.size;
                packets.push(packet);
            }
            size = i;
        }
        
        Packet{ version, type_id, value, size, packets }
    }

    fn version_sum(&self) -> usize {
        self.packets
            .iter()
            .map(|p| p.version_sum())
            .sum::<usize>() 
        + usize::from(self.version)
    }

    fn evaluate(&self) -> i64 {
        let it = self
            .packets
            .iter()
            .map(|p| p.evaluate());
        
        match self.type_id {
            0 => it.sum(),
            1 => it.product(),
            2 => it.min().unwrap(),
            3 => it.max().unwrap(),
            4 => self.value,
            5 => if self.packets[0].evaluate() > self.packets[1].evaluate() { 1 } else { 0 },
            6 => if self.packets[0].evaluate() < self.packets[1].evaluate() { 1 } else { 0 },
            7 => if self.packets[0].evaluate() == self.packets[1].evaluate() { 1 } else { 0 },
            8..=u8::MAX => unreachable!(),
        }
    }
}

/// sum of version numbers in all packets
pub fn a(input: &Vec<&str>) -> String {
    let packet = parse_input(input);
    packet.version_sum().to_string()
}

/// evaluate expression
pub fn b(input: &Vec<&str>) -> String {
    let packet = parse_input(input);
    packet.evaluate().to_string()
}

fn parse_input(input: &Vec<&str>) -> Packet {
    assert!(input.len() == 1);
    assert!(input[0].len() % 2 == 0);
    let bin = hex_to_bin(input[0]);
    Packet::from_binary(&bin)
}

fn hex_to_bin(s: &str) -> String {
    (0..s.len())
        .step_by(2)
        .map(|i| format!("{:08b}", u8::from_str_radix(&s[i..=i+1], 16).unwrap()))
        .collect::<Vec<_>>()
        .join("")
}

#[test]
pub fn test() {
    let input = vec!["8A004A801A8002F478"];
    let input2 = vec!["620080001611562C8802118E34"];
    let input3 = vec!["C0015000016115A2E0802F182340"];
    let input4 = vec!["A0016C880162017C3686B18A3D4780"];

    assert_eq!(
        hex_to_bin("D2FE28"),
        "110100101111111000101000",
    );
    assert_eq!(
        hex_to_bin("38006F45291200"),
        "00111000000000000110111101000101001010010001001000000000",
    );
    assert_eq!(
        hex_to_bin("EE00D40C823060"),
        "11101110000000001101010000001100100000100011000001100000",
    );

    let packet = parse_input(&vec!["D2FE28"]);
    assert_eq!(packet.version, 6);
    assert_eq!(packet.type_id, 4);
    assert_eq!(packet.value, 2021);
    assert_eq!(packet.version_sum(), 6);

    let packet = parse_input(&vec!["38006F45291200"]);
    assert_eq!(packet.version, 1);
    assert_eq!(packet.type_id, 6);
    assert_eq!(packet.packets.len(), 2);
    assert_eq!(packet.packets[0].type_id, 4);
    assert_eq!(packet.packets[0].value, 10);
    assert_eq!(packet.packets[1].type_id, 4);
    assert_eq!(packet.packets[1].value, 20);

    let packet = parse_input(&vec!["EE00D40C823060"]);
    assert_eq!(packet.version, 7);
    assert_eq!(packet.type_id, 3);
    assert_eq!(packet.packets.len(), 3);
    assert_eq!(packet.packets[0].type_id, 4);
    assert_eq!(packet.packets[0].value, 1);
    assert_eq!(packet.packets[1].type_id, 4);
    assert_eq!(packet.packets[1].value, 2);
    assert_eq!(packet.packets[2].type_id, 4);
    assert_eq!(packet.packets[2].value, 3);

    assert_eq!(a(&input), "16");
    assert_eq!(a(&input2), "12");
    assert_eq!(a(&input3), "23");
    assert_eq!(a(&input4), "31");

    assert_eq!(b(&vec!["C200B40A82"]), "3");
    assert_eq!(b(&vec!["04005AC33890"]), "54");
    assert_eq!(b(&vec!["880086C3E88112"]), "7");
    assert_eq!(b(&vec!["CE00C43D881120"]), "9");
    assert_eq!(b(&vec!["D8005AC2A8F0"]), "1");
    assert_eq!(b(&vec!["F600BC2D8F"]), "0");
    assert_eq!(b(&vec!["9C005AC2F8F0"]), "0");
    assert_eq!(b(&vec!["9C0141080250320F1802104A08"]), "1");
}
