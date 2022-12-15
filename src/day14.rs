#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::Result;

    fn load_rock_structures(file: &str) -> Result<Vec<Vec<u8>>> {
        let mut rocks = vec![];
        let mut max_x = 0;
        let mut max_y = 0;
        for line in BufReader::new(File::open(file)?).lines() {
            let mut structure = vec![];
            for loc in line?.split(" -> ") {
                let (x, y) = loc.split_once(",").unwrap();
                let x = x.parse::<usize>()?;
                let y = y.parse::<usize>()?;
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                structure.push((x, y));
            }
            rocks.push(structure);
        }

        max_x += 1;
        max_y += 2;
        let mut grid = vec![vec![b'.'; max_y]; max_x];
        for structure in rocks {
            for slice in structure.windows(2) {
                let slice_a = slice[0];
                let slice_b = slice[1];
                let (a_x, a_y) = slice_a;
                let (b_x, b_y) = slice_b;
                if a_x < b_x {
                    for i in a_x..=b_x {
                        grid[i][a_y] = b'#';
                    }
                } else if a_x > b_x {
                    for i in b_x..=a_x {
                        grid[i][a_y] = b'#';
                    }
                } else if a_y < b_y {
                    for i in a_y..=b_y {
                        grid[a_x][i] = b'#';
                    }
                } else if a_y > b_y {
                    for i in b_y..=a_y {
                        grid[a_x][i] = b'#';
                    }
                } else {
                    panic!("rock path seems broken: {},{} -> {},{}", a_x, a_y, b_x, b_y);
                }
            }
        }
        Ok(grid)
    }

    fn sand_units_before_abyss(mut grid: Vec<Vec<u8>>) -> usize {
        let mut count = 0;
        loop {
            let mut old = (500, 0);
            let mut sand = (500, 0);
            loop {
                sand.1 += 1;
                if sand.1 == grid[0].len() {
                    return count;
                }
                if grid[sand.0][sand.1] != b'.' {
                    if sand.0 == 0 {
                        return count;
                    }
                    sand.0 -= 1;
                    if grid[sand.0][sand.1] != b'.' {
                        sand.0 += 2;
                        if sand.0 == grid.len() {
                            return count;
                        }
                        if grid[sand.0][sand.1] != b'.' {
                            grid[old.0][old.1] = b'o';
                            break;
                        }
                    }
                }
                old = sand;
            }
            count += 1;
        }
    }

    fn sand_units_with_floor(mut grid: Vec<Vec<u8>>) -> usize {
        let origin = (500, 0);
        let mut count = 0;
        loop {
            count += 1;
            let mut old = origin;
            let mut sand = origin;
            loop {
                sand.1 += 1;
                if sand.1 == grid[0].len() {
                    grid[old.0][old.1] = b'o';
                    break;
                }
                if grid[sand.0][sand.1] != b'.' {
                    if sand.0 == 0 {
                        panic!("Fully left!!!");
                    }
                    sand.0 -= 1;
                    if grid[sand.0][sand.1] != b'.' {
                        sand.0 += 2;
                        if sand.0 == grid.len() {
                            grid.push(vec![b'.'; grid[0].len()]);
                        }
                        if grid[sand.0][sand.1] != b'.' {
                            if old == origin {
                                return count;
                            }
                            grid[old.0][old.1] = b'o';
                            break;
                        }
                    }
                }
                old = sand;
            }
        }
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Day 14 part 1: {}", sand_units_before_abyss(load_rock_structures("day14.txt")?));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Day 14 part 2: {}", sand_units_with_floor(load_rock_structures("day14.txt")?));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(24, sand_units_before_abyss(load_rock_structures("day14ex.txt")?));
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(93, sand_units_with_floor(load_rock_structures("day14ex.txt")?));
        Ok(())
    }
}
