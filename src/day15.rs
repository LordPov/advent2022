#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::Result;

    struct SensorBeacon {
        s_x: i64,
        s_y: i64,
        b_x: i64,
        b_y: i64,
        md: i64,
    }

    impl SensorBeacon {
        fn new(s_x: i64, s_y: i64, b_x: i64, b_y: i64) -> Self {
            let md = manhattan_distance((s_x, s_y), (b_x, b_y));
            SensorBeacon { s_x, s_y, b_x, b_y, md }
        }
    }

    impl TryFrom<&str> for SensorBeacon {
        type Error = anyhow::Error;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let line = value.replace("Sensor at x=", "").replace(" y=", "").replace(" closest beacon is at x=", "");
            let (sensor, beacon) = line.split_once(":").unwrap();
            let (s_x, s_y) = sensor.split_once(",").unwrap();
            let (b_x, b_y) = beacon.split_once(",").unwrap();
            Ok(SensorBeacon::new(s_x.parse::<i64>()?, s_y.parse::<i64>()?, b_x.parse::<i64>()?, b_y.parse::<i64>()?))
        }
    }

    fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    fn load_sensors_and_beacons(file: &str) -> Result<Vec<SensorBeacon>> {
        Ok(BufReader::new(File::open(file)?).lines().map(|l| l.unwrap().as_str().try_into().unwrap()).collect())
    }

    fn positions_where_beacon_not_present(sbs: &Vec<SensorBeacon>, y: i64) -> i64 {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut max_md = 0;
        for sb in sbs {
            min_x = min_x.min(sb.s_x).min(sb.b_x);
            max_x = max_x.max(sb.s_x).max(sb.b_x);
            max_md = max_md.max(sb.md);
        }
        min_x -= max_md;
        max_x += max_md;

        let mut count = 0;
        for x in min_x..=max_x {
            let mut sensor_range = false;
            for sb in sbs {
                if sb.b_x == x && sb.b_y == y {
                    continue;
                }
                if !sensor_range && manhattan_distance((x, y), (sb.s_x, sb.s_y)) <= sb.md {
                    sensor_range = true;
                }
            }
            if sensor_range {
                count += 1;
            }
        }
        count
    }

    fn tuning_frequency(sbs: &Vec<SensorBeacon>, max_dimensions: i64) -> i64 {
        fn uncovered(sbs: &Vec<SensorBeacon>, x: i64, y: i64) -> bool {
            for sb in sbs {
                if manhattan_distance((x, y), (sb.s_x, sb.s_y)) <= sb.md {
                    return false;
                }
            }
            true
        }

        fn coords_valid(x: i64, y: i64, max_dimensions: i64) -> bool {
            x >= 0 && y >= 0 && x <= max_dimensions && y <= max_dimensions
        }

        for sb in sbs {
            let perimeter = sb.md + 1;

            //nw
            let mut y = sb.s_y;
            for x in (sb.s_x - perimeter)..sb.s_x {
                if coords_valid(x, y, max_dimensions) && uncovered(sbs, x, y) {
                    return 4000000 * x + y;
                }
                y -= 1;
            }
            //ne
            let mut y = sb.s_y - perimeter;
            for x in sb.s_x..(sb.s_x + perimeter) {
                if coords_valid(x, y, max_dimensions) && uncovered(sbs, x, y) {
                    return 4000000 * x + y;
                }
                y += 1;
            }
            //sw
            let mut y = sb.s_y + 1;
            for x in (sb.s_x - perimeter + 1)..sb.s_x {
                if coords_valid(x, y, max_dimensions) && uncovered(sbs, x, y) {
                    return 4000000 * x + y;
                }
                y += 1;
            }
            //se
            let mut y = sb.s_y + perimeter;
            for x in (sb.s_x - perimeter)..=sb.s_x {
                if coords_valid(x, y, max_dimensions) && uncovered(sbs, x, y) {
                    return 4000000 * x + y;
                }
                y -= 1;
            }
        }
        0
    }

    #[test]
    fn part_1() -> Result<()> {
        //commented out as it takes several seconds to run
        // println!("Day 15 part 1: {}", positions_where_beacon_not_present(&load_sensors_and_beacons("day15.txt")?, 2000000));
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        //commented out as it takes several seconds to run
        // println!("Day 15 part 2: {}", tuning_frequency(&load_sensors_and_beacons("day15.txt")?, 4000000));
        Ok(())
    }

    #[test]
    fn part_1_test() -> Result<()> {
        assert_eq!(26, positions_where_beacon_not_present(&load_sensors_and_beacons("day15ex.txt")?, 10));
        Ok(())
    }

    #[test]
    fn part_2_test() -> Result<()> {
        assert_eq!(56000011, tuning_frequency(&load_sensors_and_beacons("day15ex.txt")?, 20));
        Ok(())
    }
}
