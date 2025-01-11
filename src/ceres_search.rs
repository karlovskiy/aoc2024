pub fn part_one(data: &[u8]) -> u64 {
    let mut result = 0;
    let mut m: i32;
    let mut a: i32;
    let mut s: i32;
    let n = calculate_n_size(data);
    let len = data.len() as i32;
    for x in 0..len {
        if data[x as usize] == b'X' {
            // right
            m = x + 1;
            if m < len && data[m as usize] == b'M' {
                a = m + 1;
                if a < len && data[a as usize] == b'A' {
                    s = a + 1;
                    if s < len && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
            // left
            m = x - 1;
            if m >= 0 && data[m as usize] == b'M' {
                a = m - 1;
                if a >= 0 && data[a as usize] == b'A' {
                    s = a - 1;
                    if s >= 0 && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
            // down
            m = x + n + 1;
            if m < len && data[m as usize] == b'M' {
                a = m + n + 1;
                if a < len && data[a as usize] == b'A' {
                    s = a + n + 1;
                    if s < len && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
            // right-down
            m = x + n + 2;
            if m < len && data[m as usize] == b'M' {
                a = m + n + 2;
                if a < len && data[a as usize] == b'A' {
                    s = a + n + 2;
                    if s < len && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
            // left-down
            m = x + n;
            if m < len && data[m as usize] == b'M' {
                a = m + n;
                if a < len && data[a as usize] == b'A' {
                    s = a + n;
                    if s < len && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
            // up
            m = x - n - 1;
            if m >= 0 && data[m as usize] == b'M' {
                a = m - n - 1;
                if a >= 0 && data[a as usize] == b'A' {
                    s = a - n - 1;
                    if s >= 0 && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
            // right up
            m = x - n;
            if m >= 0 && data[m as usize] == b'M' {
                a = m - n;
                if a >= 0 && data[a as usize] == b'A' {
                    s = a - n;
                    if s >= 0 && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
            // left up
            m = x - n - 2;
            if m >= 0 && data[m as usize] == b'M' {
                a = m - n - 2;
                if a >= 0 && data[a as usize] == b'A' {
                    s = a - n - 2;
                    if s >= 0 && data[s as usize] == b'S' {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

pub fn part_two(data: &[u8]) -> u64 {
    let mut result = 0;
    let mut m1: i32;
    let mut m2: i32;
    let mut s1: i32;
    let mut s2: i32;
    let n = calculate_n_size(data);
    let len = data.len() as i32;
    for a in 0..len {
        if data[a as usize] == b'A' {
            // M S
            //  A
            // M S
            m1 = a - n - 2;
            if m1 >= 0 && data[m1 as usize] == b'M' {
                m2 = a + n;
                if m2 < len && data[m2 as usize] == b'M' {
                    s1 = a - n;
                    if s1 >= 0 && data[s1 as usize] == b'S' {
                        s2 = a + n + 2;
                        if s2 < len && data[s2 as usize] == b'S' {
                            result += 1;
                        }
                    }
                }
            }
            // M M
            //  A
            // S S
            m1 = a - n - 2;
            if m1 >= 0 && data[m1 as usize] == b'M' {
                s1 = a + n;
                if s1 < len && data[s1 as usize] == b'S' {
                    m2 = a - n;
                    if m2 >= 0 && data[m2 as usize] == b'M' {
                        s2 = a + n + 2;
                        if s2 < len && data[s2 as usize] == b'S' {
                            result += 1;
                        }
                    }
                }
            }
            // S S
            //  A
            // M M
            s1 = a - n - 2;
            if s1 >= 0 && data[s1 as usize] == b'S' {
                m1 = a + n;
                if m1 < len && data[m1 as usize] == b'M' {
                    s2 = a - n;
                    if s2 >= 0 && data[s2 as usize] == b'S' {
                        m2 = a + n + 2;
                        if m2 < len && data[m2 as usize] == b'M' {
                            result += 1;
                        }
                    }
                }
            }
            // S M
            //  A
            // S M
            s1 = a - n - 2;
            if s1 >= 0 && data[s1 as usize] == b'S' {
                s2 = a + n;
                if s2 < len && data[s2 as usize] == b'S' {
                    m1 = a - n;
                    if m1 >= 0 && data[m1 as usize] == b'M' {
                        m2 = a + n + 2;
                        if m2 < len && data[m2 as usize] == b'M' {
                            result += 1;
                        }
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
            return i as i32;
        }
    }
    panic!("Bad data format");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_bytes!("testdata/ceres_search/example.txt");
        let total = part_one(data);
        assert_eq!(total, 18);
    }

    #[test]
    fn part_one_full() {
        let data = include_bytes!("testdata/ceres_search/input.txt");
        let total = part_one(data);
        assert_eq!(total, 2434);
    }

    #[test]
    fn part_two_example() {
        let data = include_bytes!("testdata/ceres_search/example.txt");
        let total = part_two(data);
        assert_eq!(total, 9);
    }

    #[test]
    fn part_two_full() {
        let data = include_bytes!("testdata/ceres_search/input.txt");
        let total = part_two(data);
        assert_eq!(total, 1835);
    }
}
