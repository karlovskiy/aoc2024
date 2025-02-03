use std::collections::HashSet;

pub fn part_one(data: &[u8]) -> u64 {
    let len = data.len() as i32;
    let mut antinodes_set: HashSet<i32> = HashSet::new();
    let mut antinode;
    let mut antinode_col;
    let mut x_col;
    let mut y_col;
    let n = calculate_n_size(data);
    for i in 0..len {
        let x = data[i as usize];
        if x == b'.' || x == b'\n' {
            continue;
        }
        x_col = i % (n + 1);
        for j in i + 1..len {
            let y = data[j as usize];
            if y == b'.' || y == b'\n' {
                continue;
            }
            if x == y {
                y_col = j % (n + 1);
                // first antinode
                antinode = 2 * i - j;
                antinode_col = antinode % (n + 1);
                if antinode >= 0
                    && antinode < len
                    && data[antinode as usize] != b'\n'
                    /* first antinode col depends on x and y colocation */
                    && (x_col <= y_col && antinode_col <= x_col
                        || x_col >= y_col && antinode_col >= x_col)
                {
                    antinodes_set.insert(antinode);
                }
                // second antinode
                antinode = 2 * j - i;
                antinode_col = antinode % (n + 1);
                if antinode >= 0
                    && antinode < len
                    && data[antinode as usize] != b'\n'
                    /* second antinode col depends on x and y colocation */
                    && (x_col <= y_col && antinode_col >= y_col
                        || x_col >= y_col && antinode_col <= y_col)
                {
                    antinodes_set.insert(antinode);
                }
            }
        }
    }
    antinodes_set.len() as u64
}

pub fn part_two(data: &[u8]) -> u64 {
    let len = data.len() as i32;
    let mut antinodes_set: HashSet<i32> = HashSet::new();
    let mut antinode;
    let mut col_antinode;
    let mut col_x;
    let mut col_y;
    let mut k;
    let mut diff;
    let n = calculate_n_size(data);
    for i in 0..len {
        let x = data[i as usize];
        if x == b'.' || x == b'\n' {
            continue;
        }
        col_x = i % (n + 1);
        for j in i + 1..len {
            let y = data[j as usize];
            if y == b'.' || y == b'\n' {
                continue;
            }
            if x == y {
                col_y = j % (n + 1);
                antinode = i;
                col_antinode = col_x;
                k = 1;
                diff = j - i;
                while antinode >= 0
                    && antinode < len
                    && data[antinode as usize] != b'\n'
                    && (col_x <= col_y && col_antinode <= col_x
                        || col_x >= col_y && col_antinode >= col_x)
                {
                    antinodes_set.insert(antinode);
                    antinode = i - k * diff;
                    col_antinode = antinode % (n + 1);
                    k += 1;
                }
                antinode = j;
                col_antinode = col_y;
                k = 1;
                while antinode >= 0
                    && antinode < len
                    && data[antinode as usize] != b'\n'
                    && (col_x <= col_y && col_antinode >= col_y
                        || col_x >= col_y && col_antinode <= col_y)
                {
                    antinodes_set.insert(antinode);
                    antinode = j + k * diff;
                    col_antinode = antinode % (n + 1);
                    k += 1;
                }
            }
        }
    }
    antinodes_set.len() as u64
}

#[inline]
fn calculate_n_size(data: &[u8]) -> i32 {
    for i in 0..data.len() {
        if data[i] == b'\n' {
            return i as i32;
        }
    }
    panic!("bad data format");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/resonant_collinearity/example.txt");
        let total = part_one(data);
        assert_eq!(total, 14);
    }

    #[test]
    fn part_one_full() {
        let data = include_bytes!("testdata/resonant_collinearity/input.txt");
        let total = part_one(data);
        assert_eq!(total, 369);
    }

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/resonant_collinearity/example.txt");
        let total = part_two(data);
        assert_eq!(total, 34);
    }

    #[test]
    fn part_two_full() {
        let data = include_bytes!("testdata/resonant_collinearity/input.txt");
        let total = part_two(data);
        assert_eq!(total, 1169);
    }
}
