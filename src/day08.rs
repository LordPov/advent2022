#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::Result;

    fn load_grid(file: &str) -> Result<Vec<Vec<i8>>> {
        let mut grid = vec![];
        for line in BufReader::new(File::open(file)?).lines() {
            grid.push(line?.trim().chars().map(|c| c.to_digit(10).unwrap() as i8).collect());
        }
        Ok(grid)
    }

    fn visible_trees(grid: Vec<Vec<i8>>) -> usize {
        let mut visible: HashSet<(usize, usize)> = HashSet::new();
        let mut heights: Vec<i8> = vec![];

        let max_x = grid.len();
        let max_y = grid[0].len();

        for _ in 0..max_x {
            heights.push(-1);
        }
        for x in 0..max_x {
            for y in 0..max_y {
                if grid[x][y] > heights[x] {
                    visible.insert((x, y));
                    heights[x] = grid[x][y];
                }
            }
        }

        for x in 0..max_x {
            heights[x] = -1;
        }
        for x in 0..max_x {
            for y in (0..max_y).rev() {
                if grid[x][y] > heights[x] {
                    visible.insert((x, y));
                    heights[x] = grid[x][y];
                }
            }
        }

        heights.clear();
        for _ in 0..max_y {
            heights.push(-1);
        }
        for y in 0..max_y {
            for x in 0..max_x {
                if grid[x][y] > heights[y] {
                    visible.insert((x, y));
                    heights[y] = grid[x][y];
                }
            }
        }

        for y in 0..max_y {
            heights[y] = -1;
        }
        for y in 0..max_y {
            for x in (0..max_x).rev() {
                if grid[x][y] > heights[y] {
                    visible.insert((x, y));
                    heights[y] = grid[x][y];
                }
            }
        }

        visible.len()
    }

    fn view_score(grid: &Vec<Vec<i8>>, tree_x: usize, tree_y: usize) -> u64 {
        let max_x = grid.len();
        let max_y = grid[0].len();
        let tree_h = grid[tree_x][tree_y];
        let mut score = 1;

        let mut count = 0;
        for x in (0..tree_x).rev() {
            count += 1;
            if grid[x][tree_y] >= tree_h {
                break;
            }
        }
        score *= count;

        count = 0;
        for x in (tree_x + 1)..max_x {
            count += 1;
            if grid[x][tree_y] >= tree_h {
                break;
            }
        }
        score *= count;

        count = 0;
        for y in (0..tree_y).rev() {
            count += 1;
            if grid[tree_x][y] >= tree_h {
                break;
            }
        }
        score *= count;

        count = 0;
        for y in (tree_y + 1)..max_y {
            count += 1;
            if grid[tree_x][y] >= tree_h {
                break;
            }
        }
        score *= count;

        score
    }

    fn best_view_score(grid: &Vec<Vec<i8>>) -> u64 {
        let max_x = grid.len();
        let max_y = grid[0].len();

        let mut best = 0;
        for x in 1..(max_x - 1) {
            for y in 1..(max_y - 1) {
                let score = view_score(grid, x, y);
                if score > best {
                    best = score;
                }
            }
        }
        best
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Day 08 part 1: {}", visible_trees(load_grid("day08.txt")?));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        println!("Day 08 part 2: {}", best_view_score(&load_grid("day08.txt")?));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(21, visible_trees(load_grid("day08ex.txt")?));
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        let grid = load_grid("day08ex.txt")?;
        assert_eq!(4, view_score(&grid, 1, 2));
        assert_eq!(8, view_score(&grid, 3, 2));
        Ok(())
    }
}
