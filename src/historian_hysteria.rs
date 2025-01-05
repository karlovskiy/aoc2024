use std::collections::HashMap;

pub fn part_one(lines: &Vec<String>) -> u64 {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];
    for line in lines {
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some(token) => left.push(token.to_string().parse().unwrap()),
            None => panic!("Left number not found in line"),
        }
        match tokens.next() {
            Some(token) => right.push(token.to_string().parse().unwrap()),
            None => panic!("Right number not found in line"),
        }
    }
    debug_assert_eq!(left.len(), right.len());
    left.sort();
    right.sort();
    let mut total: u64 = 0;
    for i in 0..left.len() {
        total = total + left[i].abs_diff(right[i]);
    }
    //println!("total: {total}");
    total
}

pub fn part_two(lines: &Vec<String>) -> u64 {
    let mut left_nums: Vec<u64> = vec![];
    let mut right_nums_map: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        let mut tokens = line.split_whitespace();
        let left = {
            match tokens.next() {
                Some(token) => token.to_string().parse().unwrap(),
                None => panic!("Left number not found in line"),
            }
        };
        left_nums.push(left);
        let right = {
            match tokens.next() {
                Some(token) => token.to_string().parse().unwrap(),
                None => panic!("Right number not found in line"),
            }
        };
        let count = right_nums_map.entry(right).or_insert(0);
        *count += 1;
    }
    let mut total: u64 = 0;
    for num in left_nums {
        match right_nums_map.get(&num) {
            Some(v) => total = total + v * num,
            None => {}
        }
    }
    //println!("total: {total}");
    total
}
