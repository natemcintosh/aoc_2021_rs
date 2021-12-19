use std::rc::Rc;

enum PacketContents {
    LiteralValue(usize),
    Operator { op_type: u8, packets: Vec<Packet> },
}

// fn parse_literal_bytes(s: &str) -> PacketContents::LiteralValue {}

// fn parse_operator_packet(s: &str) -> PacketContents::Operator {
//     // The first character is the length type ID
//     match &s.chars().next() {
//         Some(n) => match n {
//             '0' => {
//                 // If the length type ID is 0, then the next 15 bits are a number that
//                 // represents the total length in bits of the sub-packets contained by
//                 // this packet.

//                 // Read the next 15 bits into a decimal number
//                 let length_in_bits = usize::from_str_radix(&s[1..17], 2)
//                     .expect("Could not parse the length in bits of the number of subpackets");

//                 // Get just that many bits
//                 let sub_packet_bits = &s[16..(16 + length_in_bits + 1)];

//                 // Somehow have to parse subpackets, and know how many bits are left to
//                 // parse after each one
//             }
//             '1' => {
//                 // If the length type ID is 1, then the next 11 bits are a number that
//                 // represents the number of sub-packets immediately contained by this packet.

//                 // Read the next 11 bits into a decimal number
//                 let number_of_subpackets = usize::from_str_radix(&s[1..12], 2)
//                     .expect("Could not parse the number of subpackets");

//                 // Continually parse subpackets until reaching `number_of_subpackets`
//             }
//             _ => panic!("length type ID was not 0 or 1"),
//         },
//         None => panic!("Could not get first character while parsing operator packet"),
//     }
// }

struct Packet {
    version: u8,
    // Have to use reference counter (Rc) to keep track of when Content should be deleted
    // because PacketContents::Operator contains Packets
    content: Rc<PacketContents>,
}

// fn parse_packet(s: &str) -> Packet {
//     // The three bits are the packet version
//     let version = u8::from_str_radix(&s[..3], 2).expect("Could not parse packet version");

//     // The next three bits are the packet type ID
//     let typeid = u8::from_str_radix(&s[3..6], 2).expect("Could not parse type ID");

//     // Match the typeid, and produce the correct PacketContents
//     let content: Rc<PacketContents> = match typeid {
//         4 => Rc::new(parse_literal_bytes(&s[6..])),
//         v => Rc::new(PacketContents::Operator {
//             op_type: v,
//             packets: parse_operator_packet(&s[6..]),
//         }),
//     };

//     Packet { version, content }
// }

fn hex_to_binary(hex: &str) -> String {
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

fn to_decimal(s: &str) -> usize {
    usize::from_str_radix(s, 2).expect("Could not parse binary digit")
}

fn main() {
    let s = "110100101111111000101000";
    let version = u8::from_str_radix(&s[..3], 2).expect("Could not parse packet version");
    dbg!(version);
}

#[test]
fn test_hex_parse_1() {
    let hex = "D2FE28";
    let expected = "110100101111111000101000";
    let got = hex_to_binary(hex);
    assert_eq!(expected, &got);
}

#[test]
fn test_hex_parse_2() {
    let hex = "38006F45291200";
    let expected = "00111000000000000110111101000101001010010001001000000000";
    let got = hex_to_binary(hex);
    assert_eq!(expected, &got);
}

#[test]
fn test_hex_parse_3() {
    let hex = "EE00D40C823060";
    let expected = "11101110000000001101010000001100100000100011000001100000";
    let got = hex_to_binary(hex);
    assert_eq!(expected, &got);
}

// #[test]
// fn test_parse_packet_literal() {
//     let bin = "110100101111111000101000";
//     let Content: Rc<PacketContents> = Rc::new(PacketContents::LiteralValue(2021));
//     let expected = Packet {
//         version: 6,
//         content: Content,
//     };

//     let got = parse_packet(bin);
// }
