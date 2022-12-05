#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::Result;

    fn find_bounds(desc: &str) -> Result<(u32, u32)> {
        let (lower, upper) = desc.split_once("-").unwrap();
        Ok((lower.parse::<u32>()?, upper.parse::<u32>()?))
    }

    #[test]
    fn part_1() -> Result<()> {
        let mut overlap = 0;
        for line in BufReader::new(File::open("day04.txt").unwrap()).lines() {
            let line = line.unwrap();
            let (elf1, elf2) = line.trim().split_once(",").unwrap();
            let (elf1_l, elf1_u) = find_bounds(elf1)?;
            let (elf2_l, elf2_u) = find_bounds(elf2)?;
            // print!("{} - {} ; {} - {}", elf1_l, elf1_u, elf2_l, elf2_u);
            if (elf1_l >= elf2_l && elf1_u <= elf2_u) || (elf2_l >= elf1_l && elf2_u <= elf1_u) {
                overlap += 1;
                // println!(" Overlap!");
            } else {
                // print!("\n");
            }
        }
        println!("{} groups completely overlap", overlap);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let mut overlap = 0;
        for line in BufReader::new(File::open("day04.txt").unwrap()).lines() {
            let line = line.unwrap();
            let (elf1, elf2) = line.trim().split_once(",").unwrap();
            let (elf1_l, elf1_u) = find_bounds(elf1)?;
            let (elf2_l, elf2_u) = find_bounds(elf2)?;
            // print!("{} - {} ; {} - {}", elf1_l, elf1_u, elf2_l, elf2_u);
            if (elf1_l <= elf2_l && elf1_u >= elf2_l) || (elf2_l < elf1_l && elf2_u >= elf1_l) {
                overlap += 1;
                // println!(" Overlap!");
            } else {
                // print!("\n");
            }
        }
        println!("{} groups partially overlap", overlap);
        Ok(())
    }
}
