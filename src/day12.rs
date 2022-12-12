#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::Result;

    struct Map {
        grid: Vec<Vec<u8>>,
        start: (usize, usize),
        end: (usize, usize),
    }

    impl Map {
        fn neighbours(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
            let mut result = vec![];
            if pos.0 > 0 {
                let new = (pos.0 - 1, pos.1);
                if self.grid[new.0][new.1] <= (self.grid[pos.0][pos.1] + 1) {
                    result.push(new);
                }
            }
            if pos.0 < (self.grid.len() - 1) {
                let new = (pos.0 + 1, pos.1);
                if self.grid[new.0][new.1] <= (self.grid[pos.0][pos.1] + 1) {
                    result.push(new);
                }
            }
            if pos.1 > 0 {
                let new = (pos.0, pos.1 - 1);
                if self.grid[new.0][new.1] <= (self.grid[pos.0][pos.1] + 1) {
                    result.push(new);
                }
            }
            if pos.1 < (self.grid[pos.0].len() - 1) {
                let new = (pos.0, pos.1 + 1);
                if self.grid[new.0][new.1] <= (self.grid[pos.0][pos.1] + 1) {
                    result.push(new);
                }
            }
            result
        }
    }

    fn load_map(file: &str) -> Result<Map> {
        let mut start = (0usize, 0usize);
        let mut end = (0usize, 0usize);
        let mut grid = vec![];
        for (i, line) in BufReader::new(File::open(file)?).lines().enumerate() {
            let mut row = vec![];
            for (j, pos) in line?.as_bytes().into_iter().enumerate() {
                match *pos {
                    b'S' => {
                        start = (i, j);
                        row.push(b'a');
                    }
                    b'E' => {
                        end = (i, j);
                        row.push(b'z');
                    }
                    other => row.push(other),
                }
            }
            grid.push(row);
        }
        Ok(Map { grid, start, end })
    }

    fn shortest_path(map: &Map, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut visited = HashSet::new();
        let mut paths = vec![vec![start]];
        while !paths.is_empty() {
            let mut new_paths = vec![];
            for path in paths {
                for neighbour in map.neighbours(path.last().unwrap()).into_iter() {
                    if !visited.contains(&neighbour) {
                        let mut path = path.clone();
                        path.push(neighbour.clone());
                        if neighbour == map.end {
                            return Some(path);
                        }
                        visited.insert(neighbour);
                        new_paths.push(path);
                    }
                }
            }
            paths = new_paths;
        }
        None
    }

    fn shortest_of_all_as(map: &Map) -> Option<Vec<(usize, usize)>> {
        let mut shortest: Option<Vec<(usize, usize)>> = None;
        for i in 0..map.grid.len() {
            for j in 0..map.grid[0].len() {
                if map.grid[i][j] == b'a' {
                    if let Some(path) = shortest_path(&map, (i, j)) {
                        if shortest.is_none() || shortest.as_ref().unwrap().len() > path.len() {
                            shortest = Some(path);
                        }
                    }
                }
            }
        }
        shortest
    }

    #[test]
    fn part_1() -> Result<()> {
        let map = load_map("day12.txt")?;
        println!("Day 12 part 1: {}", shortest_path(&map, map.start).unwrap().len() - 1);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        //commented out as it takes a couple of seconds to run
        // println!("Day 12 part 2: \n{}", shortest_of_all_as(&load_map("day12.txt")?).unwrap().len() - 1);
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        let map = load_map("day12ex.txt")?;
        assert_eq!(31, shortest_path(&map, map.start).unwrap().len() - 1);
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(29, shortest_of_all_as(&load_map("day12ex.txt")?).unwrap().len() - 1);
        Ok(())
    }
}
