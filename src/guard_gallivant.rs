use std::collections::HashSet;

pub fn part_one(data: &[u8]) -> u64 {
    let mut positions_set = HashSet::new();
    let mut i: i32 = 0;
    let len = data.len() as i32;
    let n = calculate_n_size(data);
    let mut direction: u8 = 0;
    loop {
        if i >= len {
            break;
        }
        if i <= 0 && direction > 0 {
            break;
        }
        let mut c = data[i as usize];
        if direction > 0 {
            if c == b'\n' {
                break;
            }
            if c == b'^' || c == b'v' || c == b'>' || c == b'<' {
                c = b'.';
            }
        }
        match c {
            b'^' => {
                positions_set.insert(i);
                direction = 1;
                i = i - n;
            }
            b'>' => {
                positions_set.insert(i);
                direction = 2;
                i = i + 1;
            }
            b'v' => {
                positions_set.insert(i);
                direction = 3;
                i = i + n;
            }
            b'<' => {
                positions_set.insert(i);
                direction = 4;
                i = i - 1;
            }
            b'#' => {
                if direction == 0 {
                    i = i + 1;
                    continue;
                }
                (direction, i) = match direction {
                    1 => (2, i + n),
                    2 => (3, i - 1),
                    3 => (4, i - n),
                    4 => (1, i + 1),
                    _ => panic!("invalid direction: {direction}"),
                }
            }
            _ => {
                if direction == 0 {
                    i = i + 1;
                    continue;
                }
                positions_set.insert(i);
                i = match direction {
                    1 => i - n,
                    2 => i + 1,
                    3 => i + n,
                    4 => i - 1,
                    _ => panic!("invalid direction: {direction}"),
                }
            }
        }
    }
    positions_set.len() as u64
}

pub fn part_two(data: &[u8]) -> u64 {
    let mut result = 0;
    let mut n = 0;
    let mut guard_direction = 0;
    let mut guard_index = 0;
    let len = data.len() as i32;
    // loop over data to get start_direction, start_index and n
    for i in 0..len {
        let c = data[i as usize];
        match c {
            b'\n' => {
                if n == 0 {
                    n = i + 1;
                }
            }
            b'^' => {
                guard_direction = 1;
                guard_index = i;
                break;
            }
            b'>' => {
                guard_direction = 2;
                guard_index = i;
                break;
            }
            b'v' => {
                guard_direction = 3;
                guard_index = i;
                break;
            }
            b'<' => {
                guard_direction = 4;
                guard_index = i;
                break;
            }
            _ => {}
        }
    }
    let start_index = match guard_direction {
        1 => guard_index - n,
        2 => guard_index + 1,
        3 => guard_index + n,
        4 => guard_index - 1,
        _ => panic!("invalid guard direction: {guard_direction}"),
    };
    let mut positions_set: HashSet<u64> = HashSet::new();
    // loop over each possible new obstruction position
    for k in 0..len {
        if k == guard_index {
            continue;
        }
        let mut direction: u8 = guard_direction;
        let mut i = start_index;
        positions_set.clear();
        // loop over positions
        loop {
            if i >= len {
                break;
            }
            if i <= 0 {
                break;
            }
            let mut c = data[i as usize];
            if c == b'\n' {
                break;
            }
            // pack position and direction together as u64
            let position = (direction as u64 & 0xff) << 16 | i as u64 & 0xffff;
            if positions_set.contains(&position) {
                result += 1;
                break;
            }
            positions_set.insert(position);
            if i == k {
                c = b'#';
            }
            match c {
                b'#' => {
                    (direction, i) = match direction {
                        1 => (2, i + n),
                        2 => (3, i - 1),
                        3 => (4, i - n),
                        4 => (1, i + 1),
                        _ => panic!("invalid direction: {direction}"),
                    }
                }
                _ => {
                    i = match direction {
                        1 => i - n,
                        2 => i + 1,
                        3 => i + n,
                        4 => i - 1,
                        _ => panic!("invalid direction: {direction}"),
                    }
                }
            }
        }
    }
    result
}

#[inline]
fn calculate_n_size(data: &[u8]) -> i32 {
    for i in 0..data.len() {
        if data[i] == b'\n' {
            return (i + 1) as i32;
        }
    }
    panic!("bad data format");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/guard_gallivant/example.txt");
        let total = part_one(data);
        assert_eq!(total, 41);
    }

    #[test]
    fn part_one_full() {
        let data = include_bytes!("testdata/guard_gallivant/input.txt");
        let total = part_one(data);
        assert_eq!(total, 4939);
    }

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/guard_gallivant/example.txt");
        let total = part_two(data);
        assert_eq!(total, 6);
    }

    #[test]
    fn part_two_full() {
        let data = include_bytes!("testdata/guard_gallivant/input.txt");
        let total = part_two(data);
        assert_eq!(total, 1434);
    }
}
