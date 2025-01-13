use std::collections::HashSet;

pub fn part_one(data: &str) -> u64 {
    let mut result: u64 = 0;
    let mut order_rules_map: HashSet<u64> = HashSet::new();
    let mut rules_read = false;
    'lines_loop: for line in data.lines() {
        if !rules_read {
            if line.is_empty() {
                rules_read = true;
            } else {
                match line.split_once('|') {
                    None => {
                        panic!("bad line format")
                    }
                    Some(v) => {
                        let rule = ((v.0.parse::<u64>().unwrap() & 0xffff) << 16)
                            | (v.1.parse::<u64>().unwrap() & 0xffff);
                        order_rules_map.insert(rule);
                    }
                }
            }
            continue;
        }
        let pages = line
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        for i in 0..pages.len() {
            let page = pages[i];
            for j in 0..pages.len() {
                if i == j {
                    continue;
                }
                let other_page = pages[j];
                if j > i {
                    let rule = ((page & 0xffff) << 16) | (other_page & 0xffff);
                    match order_rules_map.get(&rule) {
                        Some(_v) => {}
                        None => {
                            continue 'lines_loop;
                        }
                    }
                }
                if j < i {
                    let rule = ((other_page & 0xffff) << 16) | (page & 0xffff);
                    match order_rules_map.get(&rule) {
                        Some(_v) => {}
                        None => {
                            break 'lines_loop;
                        }
                    }
                }
            }
        }
        let middle = pages[pages.len() / 2];
        result += middle;
    }
    result
}

pub fn part_two(data: &str) -> u64 {
    let mut result: u64 = 0;
    let mut order_rules_map: HashSet<u64> = HashSet::new();
    let mut rules_read = false;
    for line in data.lines() {
        if !rules_read {
            if line.is_empty() {
                rules_read = true;
            } else {
                match line.split_once('|') {
                    None => {
                        panic!("bad line format")
                    }
                    Some(v) => {
                        let rule = ((v.0.parse::<u64>().unwrap() & 0xffff) << 16)
                            | (v.1.parse::<u64>().unwrap() & 0xffff);
                        order_rules_map.insert(rule);
                    }
                }
            }
            continue;
        }
        let mut pages = line
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let len = pages.len();
        let mut i = 0;
        let mut j = 0;
        let mut correctly_ordered = true;
        'page_loop: while i < len {
            let page = pages[i];
            while j < len {
                if i == j {
                    j += 1;
                    continue;
                }
                let other_page = pages[j];
                if j > i {
                    let rule = ((page & 0xffff) << 16) | (other_page & 0xffff);
                    match order_rules_map.get(&rule) {
                        Some(_v) => {}
                        None => {
                            let temp = pages[j];
                            pages[j] = pages[i];
                            pages[i] = temp;
                            i = 0;
                            j = 0;
                            correctly_ordered = false;
                            continue 'page_loop;
                        }
                    }
                }
                j += 1;
            }
            i += 1;
            j = i;
        }
        if !correctly_ordered {
            let middle = pages[pages.len() / 2];
            result += middle;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_str!("testdata/print_queue/example.txt");
        let total = part_one(data);
        assert_eq!(total, 143);
    }

    #[test]
    fn part_one_full() {
        let data = include_str!("testdata/print_queue/input.txt");
        let total = part_one(data);
        assert_eq!(total, 5588);
    }

    #[test]
    fn part_two_example() {
        let data = include_str!("testdata/print_queue/example.txt");
        let total = part_two(data);
        assert_eq!(total, 123);
    }

    #[test]
    fn part_two_full() {
        let data = include_str!("testdata/print_queue/input.txt");
        let total = part_two(data);
        assert_eq!(total, 5331);
    }
}
