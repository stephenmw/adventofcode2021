pub fn problem1(input: &str) -> String {
    let packet = parser::parse(input).unwrap();
    format!("{}", sum_version(&packet))
}

pub fn problem2(input: &str) -> String {
    let packet = parser::parse(input).unwrap();
    format!("{}", packet.eval())
}

fn sum_version(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(l) => l.version as usize,
        Packet::Operator(o) => {
            o.version as usize + o.packets.iter().map(|x| sum_version(x)).sum::<usize>()
        }
    }
}

pub enum Packet {
    Operator(Operator),
    Literal(Literal),
}

impl Packet {
    fn eval(&self) -> u64 {
        match self {
            Packet::Operator(o) => o.eval(),
            Packet::Literal(l) => l.eval(),
        }
    }
}

pub struct Operator {
    version: u8,
    type_id: u8,
    packets: Vec<Packet>,
}

impl Operator {
    fn eval(&self) -> u64 {
        match self.type_id {
            0 => self.packets.iter().map(|p| p.eval()).sum(),
            1 => self.packets.iter().map(|p| p.eval()).product(),
            2 => self.packets.iter().map(|p| p.eval()).min().unwrap(),
            3 => self.packets.iter().map(|p| p.eval()).max().unwrap(),
            4 => panic!("operator with id 4 (literal)"),
            5 => {
                if self.packets[0].eval() > self.packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.packets[0].eval() < self.packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.packets[0].eval() == self.packets[1].eval() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("unknown type_id"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Literal {
    version: u8,
    value: u64,
}

impl Literal {
    fn eval(&self) -> u64 {
        self.value
    }
}

mod parser {
    use super::*;
    use nom::bits::complete::{tag, take};
    use nom::branch::alt;
    use nom::combinator::map;
    use nom::multi::length_count;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::sequence::tuple;
    use nom::Finish;
    use nom::IResult;
    use nom::Offset;

    pub fn parse(input: &str) -> Result<Packet, nom::error::Error<(Vec<u8>, usize)>> {
        let decoded = hex_decode(input);
        packet((&decoded, 0))
            .finish()
            .map(|(_, o)| o)
            .map_err(|e| nom::error::Error {
                input: (e.input.0.to_owned(), e.input.1),
                code: e.code,
            })
    }

    fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        alt((
            map(literal, |l| Packet::Literal(l)),
            map(operator, |o| Packet::Operator(o)),
        ))(input)
    }

    fn operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), Operator> {
        let version = take(3u8);
        let type_id = take(3u8);
        let subpackets = alt((
            preceded(tag(0, 1u8), operator_packets_bitlen),
            preceded(tag(1, 1u8), operator_packets_count),
        ));

        let mut operator = map(tuple((version, type_id, subpackets)), |(v, t, p)| {
            Operator {
                version: v,
                type_id: t,
                packets: p,
            }
        });

        operator(input)
    }

    fn operator_packets_bitlen(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
        let (mut rest, mut bit_length) = take::<_, usize, _, _>(15usize)(input)?;
        let mut ret = Vec::new();
        while bit_length > 0 {
            let (new_rest, p) = packet(rest)?;
            let consumed = bits_consumed(new_rest, rest);
            if consumed > bit_length {
                // TODO: better error
                return Err(nom::Err::Error(nom::error::Error::new(
                    rest,
                    nom::error::ErrorKind::Not,
                )));
            }
            rest = new_rest;
            bit_length -= consumed;
            ret.push(p);
        }
        Ok((rest, ret))
    }
    fn operator_packets_count(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
        length_count(take::<_, usize, _, _>(11usize), packet)(input)
    }

    fn bits_consumed(new: (&[u8], usize), old: (&[u8], usize)) -> usize {
        old.0.offset(new.0) * 8 + new.1 - old.1
    }

    fn literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), Literal> {
        let version = take(3u8);
        let literal_id = tag(4, 3u8);
        let mut header = terminated(version, literal_id);

        let group_prefix = take::<_, u8, _, _>(1u8);
        let group_data = take::<_, u64, _, _>(4u8);
        let mut group = tuple((group_prefix, group_data));

        let (mut rest, ver) = header(input)?;
        // TODO: fail if error after header success
        let mut value = 0;
        loop {
            // TODO: handle u64 overflow
            let (r, (prefix, v)) = group(rest)?;
            rest = r;
            value = (value << 4) + v;
            if prefix == 0 {
                break;
            }
        }

        Ok((
            rest,
            Literal {
                version: ver,
                value: value,
            },
        ))
    }

    fn hex_decode(s: &str) -> Vec<u8> {
        s.as_bytes()
            .chunks(2)
            .map(|x| (x[0], x.get(1).cloned().unwrap_or(0)))
            .map(|(a, b)| {
                (
                    (a as char).to_digit(16).unwrap() as u8,
                    (b as char).to_digit(16).unwrap() as u8,
                )
            })
            .map(|(a, b)| (a << 4) + b)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn literal_test() {
            let tests = [(
                &[0b11010010, 0b11111110, 0b00101000],
                Literal {
                    version: 6,
                    value: 2021,
                },
            )];

            for (input, expected) in tests {
                let (_, res) = literal((input, 0)).unwrap();
                assert_eq!(res, expected);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem1_test() {
        let tests = [
            ("8A004A801A8002F478", "16"),
            ("620080001611562C8802118E34", "12"),
            ("C0015000016115A2E0802F182340", "23"),
            ("A0016C880162017C3686B18A3D4780", "31"),
        ];

        for (input, expected) in tests {
            assert_eq!(problem1(input), expected)
        }
    }
    #[test]
    fn problem2_test() {
        let tests = [
            ("C200B40A82", "3"),
            ("04005AC33890", "54"),
            ("880086C3E88112", "7"),
            ("CE00C43D881120", "9"),
            ("D8005AC2A8F0", "1"),
            ("F600BC2D8F", "0"),
            ("9C005AC2F8F0", "0"),
            ("9C0141080250320F1802104A08", "1"),
        ];

        for (input, expected) in tests {
            assert_eq!(problem2(input), expected)
        }
    }
}
