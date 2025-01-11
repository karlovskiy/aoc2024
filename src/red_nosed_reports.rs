pub fn part_one(data: &str) -> u64 {
    let mut safe = 0;
    for line in data.lines() {
        let mut tokens = line.split_whitespace();
        let mut direction: i8 = 0;
        let mut prev: i64 = -1;
        loop {
            match tokens.next() {
                None => {
                    safe += 1;
                    break;
                }
                Some(v) => {
                    let num: i64 = v.parse().unwrap();
                    if direction == 0 && prev > 0 {
                        direction = if num > prev { 1 } else { -1 };
                    }
                    if direction > 0 && num <= prev || direction < 0 && num >= prev {
                        break;
                    }
                    if prev > 0 {
                        let diff = prev.abs_diff(num);
                        if diff > 3 || diff < 1 {
                            break;
                        }
                    }
                    prev = num;
                }
            }
        }
    }
    safe
}

pub fn part_two(data: &str) -> u64 {
    let mut safe = 0;
    for line in data.lines() {
        let levels: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if levels.len() < 3 {
            panic!("bad levels count")
        }

        let mut start_levels: Vec<(i64, i64, bool)> = vec![];
        start_levels.push((levels[0], levels[1], false));
        start_levels.push((levels[1], levels[2], true));
        start_levels.push((levels[0], levels[2], true));
        'start_level_loop: for start_lvl in start_levels {
            let start_diff = start_lvl.0.abs_diff(start_lvl.1);
            if start_diff > 3 || start_diff < 1 {
                continue;
            }
            let direction = if start_lvl.1 > start_lvl.0 {
                true
            } else {
                false
            };
            let mut prev: i64 = start_lvl.1;
            let mut bad = start_lvl.2;
            let start_index = if bad { 3 } else { 2 };
            for i in start_index..levels.len() {
                let num: i64 = levels[i];
                let diff = prev.abs_diff(num);
                if direction && num <= prev || !direction && num >= prev || diff > 3 || diff < 1 {
                    if bad {
                        continue 'start_level_loop;
                    } else {
                        bad = true;
                        continue;
                    }
                }
                prev = num;
            }
            safe += 1;
            break 'start_level_loop;
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_str!("testdata/red_nosed_reports/example.txt");
        let total = part_one(data);
        assert_eq!(total, 2);
    }

    #[test]
    fn part_one_full() {
        let data = include_str!("testdata/red_nosed_reports/input.txt");
        let total = part_one(data);
        assert_eq!(total, 279);
    }

    #[test]
    fn part_two_example() {
        let data = include_str!("testdata/red_nosed_reports/example.txt");
        let total = part_two(data);
        assert_eq!(total, 4);
    }

    #[test]
    fn part_two_full() {
        let data = include_str!("testdata/red_nosed_reports/input.txt");
        let total = part_two(data);
        assert_eq!(total, 343);
    }
}
