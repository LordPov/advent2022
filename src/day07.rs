#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, VecDeque};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::{anyhow, Result};

    struct FsDir {
        size: u64,
        items: BTreeMap<String, FsItem>,
    }

    impl FsDir {
        fn new() -> Self {
            FsDir {
                size: 0,
                items: BTreeMap::new(),
            }
        }

        fn load(&mut self, lines: &mut VecDeque<String>) -> Result<()> {
            while let Some(line) = lines.pop_front() {
                if line.starts_with("$ cd") {
                    let name = line[4..].trim();
                    if name == "/" {
                        Err(anyhow!("Unhandled 'cd /' command!"))?;
                    } else if name == ".." {
                        break;
                    }
                    match self.items.get_mut(name).unwrap() {
                        FsItem::File(_) => Err(anyhow!("Trying to 'cd' to file '{}'", name))?,
                        FsItem::Dir(dir) => dir.load(lines)?,
                    }
                } else if line.starts_with("$ ls") {
                    self.items.clear();
                } else if line.starts_with("dir ") {
                    self.items.insert(line.trim()[4..].to_string(), FsItem::Dir(FsDir::new()));
                } else {
                    let (size, name) = line.trim().split_once(" ").unwrap();
                    self.items.insert(name.to_string(), FsItem::File(size.parse::<u64>()?));
                }
            }
            Ok(())
        }

        fn calc_size(&mut self) {
            for item in self.items.values_mut() {
                self.size += match item {
                    FsItem::File(size) => *size,
                    FsItem::Dir(dir) => {
                        dir.calc_size();
                        dir.size
                    }
                };
            }
        }
    }

    enum FsItem {
        File(u64),
        Dir(FsDir),
    }

    impl FsItem {
        fn size(&self) -> u64 {
            match self {
                FsItem::File(size) => *size,
                FsItem::Dir(dir) => dir.size,
            }
        }
    }

    fn load_fs(file: &str) -> Result<FsItem> {
        let mut root = FsDir::new();
        let mut lines: VecDeque<String> = BufReader::new(File::open(file)?).lines().map(|l| l.unwrap()).collect();
        let first = lines.pop_front().unwrap();
        if first.trim() != "$ cd /" {
            Err(anyhow!("Unexpected first line of input: {}", first.trim()))?;
        }
        root.load(&mut lines)?;
        if !lines.is_empty() {
            Err(anyhow!("{} input lines remaining when FS processing complete!", lines.len()))?;
        }
        root.calc_size();
        Ok(FsItem::Dir(root))
    }

    fn dir_sizes(name: &str, item: &FsItem, dirs: &mut BTreeMap<String, u64>) {
        if let FsItem::Dir(dir) = item {
            dirs.insert(name.to_string(), dir.size);
            for (item_name, item) in &dir.items {
                dir_sizes(&format!("{}/{}", name, item_name).replace("//", "/"), item, dirs);
            }
        }
    }

    fn sum_size_dirs_at_most(file: &str, limit: u64) -> Result<u64> {
        let fs = load_fs(file)?;
        let mut dirs = BTreeMap::new();
        dir_sizes("/", &fs, &mut dirs);
        Ok(dirs.values().filter(|s| **s <= limit).sum())
    }

    fn find_smallest_dir_to_free(file: &str, needed: u64, total: u64) -> Result<(String, u64)> {
        let fs = load_fs(file)?;
        let to_free = needed - (total - fs.size());
        // println!("total: {}, used: {}, available: {}, to_free: {}", total, fs.size(), total - fs.size(), to_free);
        let mut dirs = BTreeMap::new();
        dir_sizes("/", &fs, &mut dirs);
        let mut dirs: Vec<(String, u64)> = dirs.into_iter().collect();
        dirs.sort_unstable_by_key(|(_, size)| *size);
        // println!("{:?}", dirs);
        Ok(dirs.iter().filter(|(_, size)| *size > to_free).map(|(name, size)| (name.to_string(), *size)).next().unwrap())
    }

    #[test]
    fn part_1() -> Result<()> {
        println!("Day 07 part 1: {}", sum_size_dirs_at_most("day07.txt", 100_000)?);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let (name, size) = find_smallest_dir_to_free("day07.txt", 30000000, 70000000)?;
        println!("Day 07 part 2: {} ({})", size, name);
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(95437, sum_size_dirs_at_most("day07ex.txt", 100_000)?);
        Ok(())
    }

    #[test]
    fn used_space() -> Result<()> {
        let fs = load_fs("day07ex.txt")?;
        assert_eq!(48381165, fs.size());
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(("/d".to_string(), 24933642), find_smallest_dir_to_free("day07ex.txt", 30000000, 70000000)?);
        Ok(())
    }

    #[test]
    fn print_tree() -> Result<()> {
        fn print_item(name: &str, item: &FsItem, indent: usize) {
            for _ in 0..indent {
                print!(" ");
            }
            match item {
                FsItem::File(size) => println!("- {} (file, size={})", name, size),
                FsItem::Dir(dir) => {
                    println!("- {} (dir, size={})", name, dir.size);
                    for (name, item) in &dir.items {
                        print_item(name, item, indent + 2);
                    }
                }
            }
        }

        let fs = load_fs("day07.txt")?;
        print_item("/", &fs, 0);
        Ok(())
    }
}
