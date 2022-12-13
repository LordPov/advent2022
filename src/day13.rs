#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::{Context, Result};
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize, PartialEq)]
    #[serde(untagged)]
    enum Data {
        List(Vec<Data>),
        Integer(u64),
    }

    impl TryFrom<&str> for Data {
        type Error = anyhow::Error;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            serde_json::from_str(value).with_context(|| format!("deserializing: {}", value))
        }
    }

    fn in_order(a: &Data, b: &Data) -> Ordering {
        match a {
            Data::List(l1) => match b {
                Data::List(l2) => {
                    let l1_len = l1.len();
                    let l2_len = l2.len();

                    for (d1, d2) in l1.into_iter().zip(l2.into_iter()) {
                        match in_order(d1, d2) {
                            Ordering::Equal => {},
                            other => return other,
                        }
                    }

                    if l1_len < l2_len { Ordering::Less } else if l1_len == l2_len { Ordering::Equal } else { Ordering::Greater }
                }
                Data::Integer(i2) => in_order(&Data::List(l1.clone()), &Data::List(vec![Data::Integer(*i2)])),
            }
            Data::Integer(i1) => match b {
                Data::List(l2) => in_order(&Data::List(vec![Data::Integer(*i1)]), &Data::List(l2.clone())),
                Data::Integer(i2) => if i1 < i2 { Ordering::Less } else if i1 == i2 { Ordering::Equal } else { Ordering::Greater },
            }
        }
    }

    fn are_packets_in_order(packet_pairs: Vec<(Data, Data)>) -> Vec<bool> {
        packet_pairs.iter().map(|(p1, p2)| in_order(p1, p2) != Ordering::Greater).collect()
    }

    fn decoder_key(packets: &mut Vec<Data>) -> usize {
        let divider_1 = Data::List(vec![Data::List(vec![Data::Integer(2)])]);
        let divider_2 = Data::List(vec![Data::List(vec![Data::Integer(6)])]);
        packets.push(divider_1.clone());
        packets.push(divider_2.clone());
        packets.sort_unstable_by(|a, b| in_order(a, b));
        (packets.iter().position(|p| p == &divider_1).unwrap() + 1) * (packets.iter().position(|p| p == &divider_2).unwrap() + 1)
    }

    fn load_packets(file: &str) -> Result<Vec<Data>> {
        Ok(BufReader::new(File::open(file)?).lines().map(|l| l.unwrap()).filter(|l| !l.is_empty()).map(|l| Data::try_from(l.as_str()).unwrap()).collect())
    }

    fn load_packet_pairs(file: &str) -> Result<Vec<(Data, Data)>> {
        let mut pairs = vec![];
        let mut packets = load_packets(file)?.into_iter();
        while let Some(p1) = packets.next() {
            pairs.push((p1, packets.next().unwrap()));
        }
        Ok(pairs)
    }

    #[test]
    fn part_1() -> Result<()> {
        let packets = are_packets_in_order(load_packet_pairs("day13.txt")?);
        println!("Day 13 part 1: {}", packets.iter().enumerate().filter(|(_, o)| **o).map(|(i, _)| i + 1).sum::<usize>());
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Day 13 part 2: {}", decoder_key(&mut load_packets("day13.txt")?));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(13, are_packets_in_order(load_packet_pairs("day13ex.txt")?).iter().enumerate().filter(|(_, o)| **o).map(|(i, _)| i + 1).sum::<usize>());
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(140, decoder_key(&mut load_packets("day13ex.txt")?));
        Ok(())
    }
}
