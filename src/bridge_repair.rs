pub fn part_one(data: &str) -> u64 {
    let mut total = 0;
    for line in data.lines() {
        let (calibration, nums) = parse_line(line);
        let n = nums.len();
        // 2^(n-1) combinations
        let combinations = 1 << (n - 1);
        for m in 0..combinations {
            let mut result = 0;
            for i in 0..n {
                if i == 0 || (m >> (i - 1)) & 1 == 0 {
                    result += nums[i];
                } else {
                    result *= nums[i];
                }
            }
            if result == calibration {
                total += calibration;
                break;
            }
        }
    }
    total
}

pub fn part_two(data: &str) -> u64 {
    let mut total = 0;
    for line in data.lines() {
        let (calibration, nums) = parse_line(line);
        let n = nums.len();
        // 4^(n-1) combinations
        let combinations = 1 << (2 * (n - 1));
        for m in 0..combinations {
            let mut result = 0;
            for i in 0..n {
                let num = nums[i];
                if i == 0 {
                    result += num;
                    continue;
                }
                // extract 2 bits by modulo 3
                let op = ((m >> ((i - 1) * 2)) & 3) % 3;
                match op {
                    0 => result += num,
                    1 => result *= num,
                    2 => result = result * 10u64.pow(num.ilog10() + 1) + num,
                    _ => panic!("invalid operation: {op}"),
                };
            }
            if result == calibration {
                total += calibration;
                break;
            }
        }
    }
    total
}

#[inline]
fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let parts = line.split_once(": ").unwrap();
    let calibration = parts.0.parse::<u64>().unwrap();
    let nums = parts
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    (calibration, nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let data = include_str!("testdata/bridge_repair/example.txt");
        let total = part_one(data);
        assert_eq!(total, 3749);
    }

    #[test]
    fn part_one_full() {
        let data = include_str!("testdata/bridge_repair/input.txt");
        let total = part_one(data);
        assert_eq!(total, 12553187650171);
    }

    #[test]
    fn part_two_example() {
        let data = include_str!("testdata/bridge_repair/example.txt");
        let total = part_two(data);
        assert_eq!(total, 11387);
    }

    #[test]
    fn part_two_full() {
        let data = include_str!("testdata/bridge_repair/input.txt");
        let total = part_two(data);
        assert_eq!(total, 96779702119491);
    }
}
