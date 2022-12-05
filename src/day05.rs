#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::{Context, Result};

    struct Instruction {
        count: usize,
        from: usize,
        to: usize,
    }

    fn load_input(file: &str) -> Result<(Vec<Vec<char>>, Vec<Instruction>)> {
        let mut stacks_done = false;
        let mut stacks_lines = vec![];
        let mut instructions = vec![];
        let lines = BufReader::new(File::open(file).unwrap()).lines();
        for line in lines {
            let line = line?;
            if stacks_done {
                let split: Vec<&str> = line.trim().split(" ").collect();
                instructions.push(Instruction {
                    count: split[1].parse::<usize>().with_context(|| "count")?,
                    from: split[3].parse::<usize>().with_context(|| "from")?,
                    to: split[5].parse::<usize>().with_context(|| "to")?
                });
            } else if line.trim().is_empty() {
                stacks_done = true;
            } else {
                stacks_lines.push(line);
            }
        }

        let mut stacks = vec![];
        let stack_count = stacks_lines.pop().unwrap().trim().split("   ").last().unwrap().parse::<usize>().with_context(|| "stack count")?;
        for _ in 0..stack_count {
            stacks.push(vec![]);
        }
        while let Some(line) = stacks_lines.pop() {
            let bytes = line.as_bytes();
            for i in 0..stack_count {
                let offset = i * 4 + 1;
                if offset < bytes.len() && bytes[offset] != b' ' {
                    stacks[i].push(bytes[offset] as char);
                }
            }
        }

        Ok((stacks, instructions))
    }

    #[test]
    fn part_1() -> Result<()> {
        let (mut stacks, instructions) = load_input("day05.txt")?;
        for instruction in instructions {
            for _ in 0..instruction.count {
                let item = stacks[instruction.from - 1].pop().unwrap();
                stacks[instruction.to - 1].push(item);
            }
        }

        let mut tops = String::new();
        for stack in &stacks {
            tops.push(*stack.last().unwrap());
        }
        // assert_eq!(tops, "CMZ".to_string());
        assert_eq!(tops, "SHMSDGZVC".to_string());
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let (mut stacks, instructions) = load_input("day05.txt")?;
        for instruction in instructions {
            let stack = stacks.get_mut(instruction.from - 1).unwrap();
            let mut items: Vec<char> = stack.drain((stack.len() - instruction.count)..).collect();
            stacks[instruction.to - 1].append(&mut items);
        }
        let mut tops = String::new();
        for stack in &stacks {
            tops.push(*stack.last().unwrap());
        }
        println!("Final top of stacks: {}", tops);
        // assert_eq!(tops, "MCD".to_string());
        assert_eq!(tops, "VRZGHDFBQ".to_string());
        Ok(())
    }
}
