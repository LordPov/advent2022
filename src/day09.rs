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
        let mut knots_x = vec![];
        let mut knots_y = vec![];
        for _ in 0..knots {
            knots_x.push(0i64);
            knots_y.push(0i64);
        }
        let mut tail_positions = HashSet::from([(*knots_x.last().unwrap(), *knots_y.last().unwrap())]);

        for direction in path {
            let (mut head_x, mut head_y) = match direction {
                Direction::UP(count) => (0, *count),
                Direction::DOWN(count) => (0, -*count),
                Direction::LEFT(count) => (-*count, 0),
                Direction::RIGHT(count) => (*count, 0),
            };
            while (head_x, head_y) != (0, 0) {
                if head_x > 0 {
                    head_x -= 1;
                    knots_x[0] += 1;
                } else if head_x < 0 {
                    head_x += 1;
                    knots_x[0] -= 1;
                } else if head_y > 0 {
                    head_y -= 1;
                    knots_y[0] += 1;
                } else if head_y < 0 {
                    head_y += 1;
                    knots_y[0] -= 1;
                }
                for i in 1..knots {
                    if !is_touching(knots_x[i - 1], knots_y[i - 1], knots_x[i], knots_y[i]) {
                        let (x, y) = move_follower(knots_x[i - 1], knots_y[i - 1], knots_x[i], knots_y[i]);
                        knots_x[i] += x;
                        knots_y[i] += y;
                    }
                }
                tail_positions.insert((*knots_x.last().unwrap(), *knots_y.last().unwrap()));
            }
        }
        tail_positions.len()
    }

    fn is_touching(head_x: i64, head_y: i64, tail_x: i64, tail_y: i64) -> bool {
        (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1
    }

    fn move_follower(head_x: i64, head_y: i64, tail_x: i64, tail_y: i64) -> (i64, i64) {
        ((head_x - tail_x).clamp(-1, 1), (head_y - tail_y).clamp(-1, 1))
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
