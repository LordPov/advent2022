#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::{anyhow, Result};

    const PACKET_SEQ_LEN: usize = 4;
    const MESSAGE_SEQ_LEN: usize = 14;

    fn find_start_of_sequence(buffer: &[u8], length: usize) -> Result<usize> {
        if buffer.len() < length {
            Err(anyhow!("Message is not long enough to find start of sequence: {} (req {})", buffer.len(), length))?;
        }
        for i in 0..=(buffer.len() - length) {
            let mut good = true;
            for j in 0..(length-1) {
                for k in (j + 1)..length {
                    if buffer[i + j] == buffer[i + k] {
                        good = false;
                        break;
                    }
                    if !good {
                        break;
                    }
                }
            }
            if good {
                return Ok(i + length);
            }
        }
        Err(anyhow!("Start of sequence ({}) not found in: {}", length, String::from_utf8_lossy(buffer)))
    }

    #[test]
    fn part_1() -> Result<()> {
        for (i, line) in BufReader::new(File::open("day06.txt")?).lines().enumerate() {
            let line = line?;
            println!("{}: {}", i + 1, find_start_of_sequence(line.as_bytes(), PACKET_SEQ_LEN)?);
        }
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        for (i, line) in BufReader::new(File::open("day06.txt")?).lines().enumerate() {
            let line = line?;
            println!("{}: {}", i + 1, find_start_of_sequence(line.as_bytes(), MESSAGE_SEQ_LEN)?);
        }
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(4, find_start_of_sequence("abcd".as_bytes(), PACKET_SEQ_LEN)?);
        assert_eq!(7, find_start_of_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), PACKET_SEQ_LEN)?);
        assert_eq!(5, find_start_of_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), PACKET_SEQ_LEN)?);
        assert_eq!(6, find_start_of_sequence("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), PACKET_SEQ_LEN)?);
        assert_eq!(10, find_start_of_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), PACKET_SEQ_LEN)?);
        assert_eq!(11, find_start_of_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), PACKET_SEQ_LEN)?);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(19, find_start_of_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), MESSAGE_SEQ_LEN)?);
        assert_eq!(23, find_start_of_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), MESSAGE_SEQ_LEN)?);
        assert_eq!(23, find_start_of_sequence("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), MESSAGE_SEQ_LEN)?);
        assert_eq!(29, find_start_of_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), MESSAGE_SEQ_LEN)?);
        assert_eq!(26, find_start_of_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), MESSAGE_SEQ_LEN)?);
        Ok(())
    }
}
