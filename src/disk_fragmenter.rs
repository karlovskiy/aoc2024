pub fn part_one(disk_map: &[u8]) -> u64 {
    let mut blocks = create_blocks(disk_map);
    for i in (0..blocks.len()).rev() {
        let id2 = blocks[i];
        if id2 < 0 {
            continue;
        }
        for j in 0..i {
            let id1 = blocks[j];
            if id1 >= 0 {
                continue;
            }
            blocks[j] = id2;
            blocks[i] = -1;
            break;
        }
    }
    calc_checksum(blocks, true)
}

pub fn part_two(disk_map: &[u8]) -> u64 {
    let mut blocks = create_blocks(disk_map);
    let mut id = blocks[blocks.len() - 1];
    let mut size = 0;
    for i in (0..blocks.len()).rev() {
        let x = blocks[i];
        if x != id {
            if size > 0 {
                let mut free_size = 0;
                for j in 0..(i + 1) {
                    let y = blocks[j];
                    if y >= 0 {
                        free_size = 0;
                        continue;
                    }
                    free_size += 1;
                    if free_size == size {
                        for k in 0..size {
                            blocks[j - k] = id;
                            blocks[i + 1 + k] = -1;
                        }
                        break;
                    }
                }
                id -= 1;
                size = 0;
            }
        }
        if x == id {
            size += 1;
        }
    }
    calc_checksum(blocks, false)
}

fn create_blocks(data: &[u8]) -> Vec<i32> {
    let mut blocks: Vec<i32> = vec![];
    let mut i = 0;
    for x in 0..data.len() {
        let c = data[x] as char;
        if c == '\n' {
            break;
        }
        if c == '0' {
            continue;
        }
        let n = c.to_digit(10).unwrap() as u8;
        let digit;
        if x % 2 == 0 {
            digit = i;
            i += 1;
        } else {
            digit = -1;
        }
        for _ in 0..n {
            blocks.push(digit);
        }
    }
    blocks
}

fn calc_checksum(blocks: Vec<i32>, break_on_negative: bool) -> u64 {
    let mut result: u64 = 0;
    for i in 0..blocks.len() {
        let x = blocks[i];
        if x < 0 {
            if break_on_negative {
                break;
            }
            continue;
        }
        result += i as u64 * x as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let disk_map = include_bytes!("testdata/disk_fragmenter/example.txt");
        let checksum = part_one(disk_map);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn part_one_full() {
        let disk_map = include_bytes!("testdata/disk_fragmenter/input.txt");
        let checksum = part_one(disk_map);
        assert_eq!(checksum, 6337367222422);
    }

    #[test]
    fn part_two_example() {
        let disk_map = include_bytes!("testdata/disk_fragmenter/example.txt");
        let checksum = part_two(disk_map);
        assert_eq!(checksum, 2858);
    }

    #[test]
    fn part_two_full() {
        let disk_map = include_bytes!("testdata/disk_fragmenter/input.txt");
        let checksum = part_two(disk_map);
        assert_eq!(checksum, 6361380647183);
    }
}
