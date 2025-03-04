use std::collections::HashSet;

pub fn part_one(data: &[u8]) -> u64 {
    let n = calc_size(data);
    let mut positions = HashSet::new();
    let mut score = 0;
    for i in 0..data.len() {
        let x = data[i];
        if x as char == '0' {
            check_trail_score(data, n, &mut positions, i, x);
            score += positions.len() as u64;
            positions.clear();
        }
    }
    score
}

pub fn part_two(data: &[u8]) -> u64 {
    let n = calc_size(data);
    let mut rating = 0;
    for i in 0..data.len() {
        let x = data[i];
        if x as char == '0' {
            check_trail_rating(data, n, &mut rating, i, x);
        }
    }
    rating
}

fn check_trail_rating(data: &[u8], n: usize, rating: &mut u64, index: usize, height: u8) {
    if data[index] != height {
        return;
    }
    if height as char == '9' {
        *rating = *rating + 1;
        return;
    }
    let next_height = height + 1;
    let right = index + 1;
    if right < data.len() {
        check_trail_rating(data, n, rating, right, next_height);
    }
    if index != 0 {
        let left = index - 1;
        check_trail_rating(data, n, rating, left, next_height);
    }
    if index > n {
        let up = index - n - 1;
        check_trail_rating(data, n, rating, up, next_height);
    }
    let down = index + n + 1;
    if down < data.len() {
        check_trail_rating(data, n, rating, down, next_height);
    }
}

fn check_trail_score(
    data: &[u8],
    n: usize,
    positions: &mut HashSet<usize>,
    index: usize,
    height: u8,
) {
    if data[index] != height {
        return;
    }
    if height as char == '9' {
        positions.insert(index);
        return;
    }
    let next_height = height + 1;
    let right = index + 1;
    if right < data.len() {
        check_trail_score(data, n, positions, right, next_height);
    }
    if index != 0 {
        let left = index - 1;
        check_trail_score(data, n, positions, left, next_height);
    }
    if index > n {
        let up = index - n - 1;
        check_trail_score(data, n, positions, up, next_height);
    }
    let down = index + n + 1;
    if down < data.len() {
        check_trail_score(data, n, positions, down, next_height);
    }
}

#[inline]
fn calc_size(data: &[u8]) -> usize {
    for i in 0..data.len() {
        if data[i] == b'\n' {
            return i;
        }
    }
    panic!("Bad data format");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/hoof_it/example.txt");
        let result = part_one(data);
        assert_eq!(result, 36);
    }

    #[test]
    fn part_one_full() {
        let data = include_bytes!("testdata/hoof_it/input.txt");
        let result = part_one(data);
        assert_eq!(result, 459);
    }

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/hoof_it/example.txt");
        let result = part_two(data);
        assert_eq!(result, 81);
    }

    #[test]
    fn part_two_full() {
        let data = include_bytes!("testdata/hoof_it/input.txt");
        let result = part_two(data);
        assert_eq!(result, 1034);
    }
}
