use std::str;

pub fn part_one(data: &[u8]) -> u64 {
    let mut total = 0;
    /*
        state mask:
        - 1 bit  (2): m
        - 2 bit  (4): u
        - 3 bit  (8): l
        - 4 bit (16): ( left num
        - 5 bit (32): , right num
    */
    let mut state: u8 = 0;
    let mut left_chars: Vec<u8> = vec![];
    let mut right_chars: Vec<u8> = vec![];
    for b in data {
        if *b == b'm' && state == 0 {
            state += 2;
            continue;
        }
        if *b == b'u' && state == 2 {
            state += 4;
            continue;
        }
        if *b == b'l' && state == 6 {
            state += 8;
            continue;
        }
        if *b == b'(' && state == 14 {
            state += 16;
            left_chars.clear();
            continue;
        }
        // left number
        if *b >= b'0' && *b <= b'9' && state == 30 {
            left_chars.push(*b);
            continue;
        }
        if *b == b',' && state == 30 {
            state += 32;
            right_chars.clear();
            continue;
        }
        // right number
        if *b >= b'0' && *b <= b'9' && state == 62 {
            right_chars.push(*b);
            continue;
        }
        if *b == b')' && state == 62 {
            let l: u64 = str::from_utf8(&left_chars).unwrap().parse().unwrap();
            let r: u64 = str::from_utf8(&right_chars).unwrap().parse().unwrap();
            total += l * r;
        }
        state = 0;
    }
    total
}

pub fn part_two(data: &[u8]) -> u64 {
    let mut total = 0;
    /*
        multiply command state mask:
        - 1 bit  (2): m
        - 2 bit  (4): u
        - 3 bit  (8): l
        - 4 bit (16): ( left num
        - 5 bit (32): , right num
    */
    let mut mul_state: u8 = 0;
    /*
        enable command state mask:
        - 1 bit (2): d
        - 2 bit (4): o
        - 3 bit (8): (
    */
    let mut enabled_state: u8 = 0;
    /*
        disable command state mask:
        - 1 bit  (2): d
        - 2 bit  (4): o
        - 3 bit  (8): n
        - 4 bit (16): '
        - 4 bit (16): t
        - 5 bit (32): (
    */
    let mut disabled_state: u8 = 0;
    let mut left_chars: Vec<u8> = vec![];
    let mut right_chars: Vec<u8> = vec![];
    let mut enabled = true;
    for b in data {
        if !enabled {
            let upd_enabled_state = is_enabled(enabled_state, *b);
            if upd_enabled_state > 0 {
                enabled_state = upd_enabled_state;
                continue;
            }
            if *b == b')' && enabled_state == 14 {
                enabled = true;
                disabled_state = 0;
            }
            enabled_state = 0;
            continue;
        }
        let upd_disabled_state = is_disabled(disabled_state, *b);
        if upd_disabled_state > 0 {
            disabled_state = upd_disabled_state;
            continue;
        }
        if *b == b')' && disabled_state == 126 {
            enabled = false;
            enabled_state = 0;
            continue;
        }
        disabled_state = 0;
        let upd_mul_state = is_mul(mul_state, *b);
        if upd_mul_state > 0 {
            mul_state = upd_mul_state;
            continue;
        }
        // (
        if *b == b'(' && mul_state == 14 {
            mul_state += 16;
            left_chars.clear();
            continue;
        }
        // left number
        if *b >= b'0' && *b <= b'9' && mul_state == 30 {
            left_chars.push(*b);
            continue;
        }
        if *b == b',' && mul_state == 30 {
            mul_state += 32;
            right_chars.clear();
            continue;
        }
        // right number
        if *b >= b'0' && *b <= b'9' && mul_state == 62 {
            right_chars.push(*b);
            continue;
        }
        if *b == b')' && mul_state == 62 {
            let l: u64 = str::from_utf8(&left_chars).unwrap().parse().unwrap();
            let r: u64 = str::from_utf8(&right_chars).unwrap().parse().unwrap();
            total += l * r;
        }
        mul_state = 0;
    }
    total
}

fn is_mul(mut state: u8, b: u8) -> u8 {
    if b == b'm' && state == 0 {
        state += 2;
        return state;
    }
    if b == b'u' && state == 2 {
        state += 4;
        return state;
    }
    if b == b'l' && state == 6 {
        state += 8;
        return state;
    }
    0
}

fn is_enabled(mut state: u8, b: u8) -> u8 {
    if b == b'd' && state == 0 {
        state += 2;
        return state;
    }
    if b == b'o' && state == 2 {
        state += 4;
        return state;
    }
    if b == b'(' && state == 6 {
        state += 8;
        return state;
    }
    0
}

fn is_disabled(mut state: u8, b: u8) -> u8 {
    if b == b'd' && state == 0 {
        state += 2;
        return state;
    }
    if b == b'o' && state == 2 {
        state += 4;
        return state;
    }
    if b == b'n' && state == 6 {
        state += 8;
        return state;
    }
    if b == b'\'' && state == 14 {
        state += 16;
        return state;
    }
    if b == b't' && state == 30 {
        state += 32;
        return state;
    }
    if b == b'(' && state == 62 {
        state += 64;
        return state;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_simple() {
        let data =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".as_bytes();
        let total = part_one(data);
        assert_eq!(total, 161);
    }

    #[test]
    fn part_one_full() {
        let data = include_bytes!("testdata/mull_it_over/input.txt");
        let total = part_one(data);
        assert_eq!(total, 173731097);
    }

    #[test]
    fn part_two_simple() {
        let data =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".as_bytes();
        let total = part_two(data);
        assert_eq!(total, 48);
    }

    #[test]
    fn part_two_full() {
        let data = include_bytes!("testdata/mull_it_over/input.txt");
        let total = part_two(data);
        assert_eq!(total, 93729253);
    }
}
