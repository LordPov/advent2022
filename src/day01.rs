#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn elves_calories() -> Vec<i32> {
        let file = File::open("day01.txt").unwrap();
        let reader = BufReader::new(file);

        let mut calories = 0;
        let mut elves = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            let trimmed = line.trim();
            if trimmed.is_empty() {
                elves.push(calories);
                calories = 0;
            } else {
                calories += trimmed.parse::<i32>().unwrap();
            }
        }
        elves.push(calories);
        elves
    }

    #[test]
    fn part_1() {
        let mut max = (0, 0);
        for (i, calories) in elves_calories().iter().enumerate() {
            let elf = i + 1;
            // println!("{}: {}", elf, calories);
            if calories > &max.1 {
                max = (elf, *calories);
            }
        }
        println!("Elf {} has the most calories: {}", max.0, max.1);
    }

    #[test]
    fn part_2() {
        let mut elves = elves_calories();
        elves.sort();
        elves.reverse();
        println!("Top 3 combined: {}", elves[0] + elves[1] + elves[2]);
    }
}
