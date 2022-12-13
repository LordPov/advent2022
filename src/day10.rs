#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::Result;

    enum Instruction {
        NOOP,
        ADDX(i64),
    }

    impl From<&str> for Instruction {
        fn from(input: &str) -> Self {
            match &input[..4] {
                "noop" => Instruction::NOOP,
                "addx" => Instruction::ADDX(input[5..].parse::<i64>().unwrap()),
                other => panic!("Invalid instruction: {}", other),
            }
        }
    }

    fn load_instructions(file: &str) -> Result<Vec<Instruction>> {
        let mut instructions = vec![];
        for line in BufReader::new(File::open(file)?).lines() {
            instructions.push(line?.trim().into());
        }
        Ok(instructions)
    }

    fn process_instructions(instructions: &Vec<Instruction>) -> Vec<i64> {
        let mut x = 1i64;
        let mut result = vec![x];
        for instruction in instructions {
            let (cycles, change) = match instruction {
                Instruction::NOOP => (1, 0),
                Instruction::ADDX(v) => (2, *v),
            };
            result.push(x);
            for _ in 1..cycles {
                result.push(x);
            }
            x += change;
        }
        result
    }

    fn find_signal_strengths(instructions: &Vec<Instruction>, interesting: Vec<usize>) -> Vec<i64> {
        let xs = process_instructions(instructions);
        interesting.iter().map(|c| xs[*c] * (*c as i64)).collect()
    }

    fn print_screen(instructions: &Vec<Instruction>) -> String {
        let xs = process_instructions(instructions);
        let mut screen = String::new();
        for i in 0..6 {
            for j in 0..40 {
                let pos = i * 40 + j + 1;
                let sprite = xs[pos];
                let pixel = j as i64;
                // println!("{} - s: {}-{}, p: {}", pos, sprite - 1, sprite + 1, pixel);
                if pixel >= (sprite - 1) && pixel <= (sprite + 1) {
                    screen.push('#');
                } else {
                    screen.push('.');
                }
            }
            screen.push('\n');
        }
        screen
    }

    #[test]
    fn part_1() -> Result<()> {
        let answer = find_signal_strengths(&load_instructions("day10.txt")?, vec![20, 60, 100, 140, 180, 220]).iter().sum::<i64>();
        println!("Day 10 part 1: {}", answer);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Day 10 part 2: \n{}", print_screen(&load_instructions("day10.txt")?));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        let strengths = find_signal_strengths(&load_instructions("day10ex.txt")?, vec![20, 60, 100, 140, 180, 220]);
        assert_eq!(vec![420, 1140, 1800, 2940, 2880, 3960], strengths);
        assert_eq!(13140i64, strengths.iter().sum::<i64>());
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let answer = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
        let screen = print_screen(&load_instructions("day10ex.txt")?);
        // print!("{}", screen);
        assert_eq!(answer, screen);
        Ok(())
    }
}
