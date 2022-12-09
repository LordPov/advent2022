#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::Result;

    enum Direction {
        UP(i64),
        DOWN(i64),
        LEFT(i64),
        RIGHT(i64),
    }

    impl From<&str> for Direction {
        fn from(input: &str) -> Self {
            let (dir, count) = input.split_once(" ").unwrap();
            let count = count.parse::<i64>().unwrap();
            match dir {
                "U" => Direction::UP(count),
                "D" => Direction::DOWN(count),
                "L" => Direction::LEFT(count),
                "R" => Direction::RIGHT(count),
                other => panic!("Unexpected direction: {}", other),
            }
        }
    }

    fn count_tail_positions(path: &Vec<Direction>, knots: usize) -> usize {
        let mut knots = vec![(0, 0); knots];
        let mut tail_positions = HashSet::from([(0i64, 0i64)]);

        for direction in path {
            let mut head = match direction {
                Direction::UP(count) => (0, *count),
                Direction::DOWN(count) => (0, -*count),
                Direction::LEFT(count) => (-*count, 0),
                Direction::RIGHT(count) => (*count, 0),
            };
            while head != (0, 0) {
                if head.0 > 0 {
                    head.0 -= 1;
                    knots[0].0 += 1;
                } else if head.0 < 0 {
                    head.0 += 1;
                    knots[0].0 -= 1;
                } else if head.1 > 0 {
                    head.1 -= 1;
                    knots[0].1 += 1;
                } else if head.1 < 0 {
                    head.1 += 1;
                    knots[0].1 -= 1;
                }
                for i in 1..knots.len() {
                    if !is_touching(knots[i - 1], knots[i]) {
                        let (x, y) = move_follower(knots[i - 1], knots[i]);
                        knots[i].0 += x;
                        knots[i].1 += y;
                    }
                }
                tail_positions.insert(*knots.last().unwrap());
            }
        }
        tail_positions.len()
    }

    fn is_touching(head: (i64, i64), tail: (i64, i64)) -> bool {
        (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
    }

    fn move_follower(head: (i64, i64), tail: (i64, i64)) -> (i64, i64) {
        ((head.0 - tail.0).clamp(-1, 1), (head.1 - tail.1).clamp(-1, 1))
    }

    fn load_path(file: &str) -> Result<Vec<Direction>> {
        let mut path = vec![];
        for line in BufReader::new(File::open(file)?).lines() {
            path.push(line?.trim().into());
        }
        Ok(path)
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Day 09 part 1: {}", count_tail_positions(&load_path("day09.txt")?, 2));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Day 09 part 2: {}", count_tail_positions(&load_path("day09.txt")?, 10));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(13, count_tail_positions(&load_path("day09ex1.txt")?, 2));
        Ok(())
    }

    #[test]
    fn part_2_test_1() -> Result<()> {
        assert_eq!(1, count_tail_positions(&load_path("day09ex1.txt")?, 10));
        Ok(())
    }

    #[test]
    fn part_2_test_2() -> Result<()> {
        assert_eq!(36, count_tail_positions(&load_path("day09ex2.txt")?, 10));
        Ok(())
    }
}
