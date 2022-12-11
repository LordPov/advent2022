#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use anyhow::Result;

    enum Operation {
        Add(Option<u64>),
        Subtract(Option<u64>),
        Multiply(Option<u64>),
        Divide(Option<u64>),
    }

    impl From<&str> for Operation {
        fn from(input: &str) -> Self {
            const PREFIX: &str = "  Operation: new = old ";
            if !input.starts_with(PREFIX) {
                panic!("Unexpected operation: {}", input);
            }
            let number = if &input[(PREFIX.len() + 2)..] == "old" { None } else { Some(input[(PREFIX.len() + 2)..].parse::<u64>().unwrap()) };
            match input.chars().nth(PREFIX.len()).unwrap() {
                '+' => Operation::Add(number),
                '-' => Operation::Subtract(number),
                '*' => Operation::Multiply(number),
                '/' => Operation::Divide(number),
                other => panic!("Unexpected operation sign '{}' ({})", other, input),
            }
        }
    }

    impl Operation {
        fn apply(&self, old: u64) -> u64 {
            match self {
                Operation::Add(number) => match number {
                    None => old + old,
                    Some(number) => old + number,
                }
                Operation::Subtract(number) => match number {
                    None => old - old,
                    Some(number) => old - number,
                }
                Operation::Multiply(number) => match number {
                    None => old * old,
                    Some(number) => old * number,
                }
                Operation::Divide(number) => match number {
                    None => old / old,
                    Some(number) => old / number,
                }
            }
        }
    }

    struct Throw {
        divisible_by: u64,
        true_monkey: usize,
        false_monkey: usize,
    }

    impl From<&[&str]> for Throw {
        fn from(input: &[&str]) -> Self {
            const TEST_PREFIX: &str = "  Test: divisible by ";
            const TRUE_PREFIX: &str = "    If true: throw to monkey ";
            const FALSE_PREFIX: &str = "    If false: throw to monkey ";

            if input[0].starts_with(TEST_PREFIX) {
                if input[1].starts_with(TRUE_PREFIX) {
                    if input[2].starts_with(FALSE_PREFIX) {
                        return Throw {
                            divisible_by: input[0][TEST_PREFIX.len()..].parse::<u64>().unwrap(),
                            true_monkey: input[1][TRUE_PREFIX.len()..].parse::<usize>().unwrap(),
                            false_monkey: input[2][FALSE_PREFIX.len()..].parse::<usize>().unwrap(),
                        };
                    }
                }
            }
            panic!("Unexpected monkey test input: {}", input.join("\n"));
        }
    }

    impl Throw {
        fn throw(&self, item: u64) -> (usize, u64) {
            if item % self.divisible_by == 0 {
                (self.true_monkey, item)
            } else {
                (self.false_monkey, item)
            }
        }
    }

    struct Monkey {
        items: VecDeque<u64>,
        operation: Operation,
        throw: Throw,
        inspected: usize,
    }

    impl From<&str> for Monkey {
        fn from(input: &str) -> Self {
            const ITEMS_PREFIX: &str = "  Starting items: ";

            let lines = input.split("\n").collect::<Vec<&str>>();
            Monkey {
                items: lines[1][ITEMS_PREFIX.len()..].split(", ").map(|i| i.parse::<u64>().unwrap()).collect(),
                operation: lines[2].into(),
                throw: lines[3..].into(),
                inspected: 0,
            }
        }
    }

    impl Monkey {
        fn turn(&mut self, worry_divisor: u64) -> Vec<(usize, u64)> {
            let mut throws = vec![];
            while let Some(item) = self.items.pop_front() {
                self.inspected += 1;
                throws.push(self.throw.throw(self.operation.apply(item) / worry_divisor));
            }
            throws
        }
    }

    fn load_monkeys(file: &str) -> Result<Vec<Monkey>> {
        let mut monkeys = vec![];
        let mut input = String::new();
        BufReader::new(File::open(file)?).read_to_string(&mut input)?;
        for lines in input.split("\n\n") {
            monkeys.push(lines.into());
        }
        Ok(monkeys)
    }

    fn calculate_monkey_business(monkies: &mut Vec<Monkey>, rounds: usize, worry_divisor: u64) -> usize {
        let max = monkies.iter().map(|m| m.throw.divisible_by).reduce(|a, i| a * i).unwrap();
        for _round in 1..=rounds {
            for i in 0..monkies.len() {
                for (monkey, item) in monkies[i].turn(worry_divisor) {
                    monkies[monkey].items.push_back(item % max);
                }
            }
            // if round == 1 || round == 20 || round % 1000 == 0 {
            //     println!("== After round {} ==", round);
            //     for i in 0..monkies.len() {
            //         println!("Monkey {} inspected items {} times", i, monkies[i].inspected);
            //     }
            // }
        }
        monkies.sort_unstable_by_key(|m| m.inspected);
        monkies.reverse();
        monkies[0].inspected * monkies[1].inspected
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Day 11 part 1: {}", calculate_monkey_business(&mut load_monkeys("day11.txt")?, 20, 3));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Day 11 part 2: \n{}", calculate_monkey_business(&mut load_monkeys("day11.txt")?, 10000, 1));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(10605, calculate_monkey_business(&mut load_monkeys("day11ex.txt")?, 20, 3));
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(2713310158, calculate_monkey_business(&mut load_monkeys("day11ex.txt")?, 10000, 1));
        Ok(())
    }
}
