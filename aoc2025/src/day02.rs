use std::fs::File;
use std::io::{self, BufRead};

// we only read the first line of the text file
pub fn read_input(filepath: &str) -> Vec<(u128, u128)> {
    let file = File::open(filepath).expect("failed to open file");
    let reader = io::BufReader::new(file);
    let line = reader.lines().next().expect("Invalid txt file format").expect("Failed to read line");
    let mut v = Vec::new();
    for pair in line.split(",") {
        let mut v2 = Vec::new();
        for t in pair.split("-") {
            let num: u128 = t.parse().unwrap();
            v2.push(num);
        }
        if v2.len() != 2 {
            panic!("Invalid number pair");
        }
        v.push((v2[0], v2[1]));
    }
    v
}

/**
* given value num, i want to find min value res such that [res][res] >= num 
* this is essentially res * [10^k + 1] >= num
* we can potentially fix k, then solve for res
*  
* if value is single digit like 1, we can't half it....
*/
fn f(num: u128) -> u128 {
    if len(num) == 1 {
        return num;
    }
    for digits in 1..=10 {
        let base = 10u128.pow(digits) + 1;
        let min_res: u128 = (num + base - 1) / base; // ceil(num / base)
        // Check if min_res really has "digits" digits
        if min_res >= 10u128.pow(digits - 1) && min_res < 10u128.pow(digits) {
            return min_res;
        }
    }
    num
    // panic!("no valid res found");
}

fn len(num: u128) -> u128 {
    if num == 0 { return 1; }
    let mut cnt = 0;
    let mut num = num;
    while num > 0 {
        cnt += 1;
        num /= 10;
    }
    cnt
}
pub fn find_invalid_ids_in_range(start: u128, end: u128) -> Vec<u128> {
    let mut res = Vec::new();
    let mut left: u128 = f(start);
    while true {
        let val: u128 = left * (10u128.pow(len(left).try_into().unwrap()) + 1);
        if val <= end {
            res.push(val);
        } else {
            break
        }
        left += 1;
    }
    res
}

pub fn find_invalid_ids(ranges: Vec<(u128, u128)>) -> u128 {
    let mut res: u128 = 0;
    for range in ranges {
        let (start, end) = range;
        for val in find_invalid_ids_in_range(start, end) {
            res += val;
        }
    }
    res
}

/******************* part 2 **********************/

fn repeatn(s: &str, n: u32) -> String {
    let mut res = String::new();
    for _ in 0..n {
        res += s;
    }
    res
}


pub fn count_invalid_ids_in_range2(start: u128, end: u128) -> u128 {
    let mut res = 0;
    for i in start..=end {
        let s = i.to_string();
        let len = s.len();
        for j in 1..=len/2 {
            if len % j == 0 {
                let repeat_time = len / j;
                let part = &s[..j];
                if part.repeat(repeat_time) == s {
                    res += i;
                    break;
                }
            }
        }
    }
    res
}

pub fn count_invalid_ids2(ranges: Vec<(u128, u128)>) -> u128 {
    let mut res: u128 = 0;
    for range in ranges {
        let (start, end) = range;
        res += count_invalid_ids_in_range2(start, end);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_one() {
        assert_eq!("abcabc", repeatn("abc", 2));
    }

    #[test]
    fn test_read_input() {
        let exp = vec![(11,22), (95,115), (998, 1012), (1188511880, 1188511890), (222220, 222224), (1698522, 1698528), (446443, 446449), (38593856,38593862),(565653,565659),(824824821,824824827),(2121212118,2121212124)];
        assert_eq!(exp, read_input("inputs/day02/input1.txt"));
    }
    #[test]
    fn test_len_zero() {
        assert_eq!(1, len(0));
    }
    #[test]
    fn test_len_one() {
        assert_eq!(1, len(9));
    }
    #[test]
    fn test_len() {
        assert_eq!(3, len(123));
    }
    #[test]
    fn test_find_half_one() {
        assert_eq!(1, f(11));
    }
    #[test]
    fn test_find_half_self() {
        assert_eq!(9, f(9));
    }
    #[test]
    fn test_find_half_big_val() {
        assert_eq!(11885, f(1188511884));
    }
    #[test]
    fn test_find_invalid_ids_in_range_one() {
        assert_eq!(vec![11, 22], find_invalid_ids_in_range(11, 22));
    }
    #[test]
    fn test_find_invalid_ids_in_range_two() {
        assert_eq!(vec![99], find_invalid_ids_in_range(95, 115));
    }
    #[test]
    fn test_find_invalid_ids_in_range_three() {
        assert_eq!(vec![1010], find_invalid_ids_in_range(998, 1012));
    }

    #[test] fn test_simple() {
        let ranges = vec![(11,22), (95,115), (998, 1012), (1188511880, 1188511889), (222220, 222224), (1698522, 1698528), (446443, 446449), (38593856,38593862)];
        assert_eq!(1227775554, find_invalid_ids(ranges));
    }
    /************ part2 test case *************/

    #[test]
    fn test_count_invalid_ids_in_range_one() {
        assert_eq!(33, count_invalid_ids_in_range2(11, 22));
    }
    #[test]
    fn test_count_invalid_ids_in_range_two() {
        assert_eq!(111 + 99, count_invalid_ids_in_range2(95, 115));
    }
    #[test]
    fn test_count_invalid_ids_in_range_three() {
        assert_eq!(1010 + 999, count_invalid_ids_in_range2(998, 1012));
    }
    fn test_part2() {
        let ranges = vec![(11,22), (95,115), (998, 1012), (1188511880, 1188511889), (222220, 222224), (1698522, 1698528), (446443, 446449), (38593856,38593862)];
        assert_eq!(4174379265, count_invalid_ids2(ranges));
    }
}
