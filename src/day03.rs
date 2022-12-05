#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use anyhow::{anyhow, Result};

    fn item_to_priority(c: u8) -> Result<u32> {
        if c >= b'a' && c <= b'z' {
            Ok((c - b'a' + 1) as u32)
        } else if c >= b'A' && c <= b'Z' {
            Ok((c - b'A' + 27) as u32)
        } else {
            Err(anyhow!("Invalid item: '{}' ({})", c as char, c))?
        }
    }

    #[test]
    fn part_1() -> Result<()> {
        let mut priorities = 0;
        for line in BufReader::new(File::open("day03.txt").unwrap()).lines() {
            let line = line.unwrap();
            let trimmed = line.trim();
            let (first, second) = trimmed.split_at(trimmed.len() / 2);
            for item in first.bytes() {
                if second.bytes().any(|b| b == item) {
                    priorities += item_to_priority(item)?;
                    break;
                }
            }
        }
        println!("Sum of priorities: {}", priorities);
        Ok(())
    }


    #[test]
    fn part_2() -> Result<()> {
        let mut priorities = 0;
        let mut _group = 0;
        let mut lines = BufReader::new(File::open("day03.txt").unwrap()).lines();
        loop {
            _group += 1;
            match lines.next() {
                None => break,
                Some(line) => {
                    let elf1 = line?;
                    let elf2 = lines.next().unwrap()?;
                    let elf3 = lines.next().unwrap()?;
                    for item in elf1.trim().bytes() {
                        if elf2.as_bytes().contains(&item) && elf3.as_bytes().contains(&item) {
                            let prio = item_to_priority(item)?;
                            // println!("Group {} all have '{}', prio {}", group, item as char, prio);
                            priorities += prio;
                            break;
                        }
                    }
                }
            }
        }
        println!("Sum of priorities: {}", priorities);
        Ok(())
     }
}
