use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::cmp::max;

pub fn read_input(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("Failed to open day03 file");
    let mut buf_reader = BufReader::new(file);
    let mut res = Vec::new();
    for s in buf_reader.lines() {
        res.push(s.unwrap());
    }
    res
}

/***************************** part1 ****************************/
/*
* given a string, i want to pick two value x, y  from the string such that index(x) < index(y) and
* return the max [x][y]
*/
fn largest_pair1(s: &str) -> u32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n < 2 {
        panic!("string should contain at least two characters: {}", s);
    }
    let mut x = 0;
    let mut y = 0;
    for i in 0..n {
        let val = chars[i].to_digit(10).unwrap();
        if i != n - 1 && val > x {
            x = val;
            y = 0;
        } else if val > y {
            y = val;
        }
    }
    x * 10 + y
}

pub fn solve1(input: Vec<&str>) -> u32 {
    let mut res = 0;
    for s in input {
        res += largest_pair1(s);
    }
    res
}

/******************************* part2 *************************/

fn largest_rel_k(s: &str, k: usize) -> u64 {
    let n = s.len();
    if n < k {
        panic!("string dont have enough letters: {n} < {k}");
    }
    let chars: Vec<u64> = s.chars().map(|s| s.to_digit(10).expect("string contain non digit") as u64).collect();
    let mut dp: Vec<u64> = vec![0; k];
    for i in 0..n {
        for j in (1..k).rev() {
            dp[j] = max(dp[j], dp[j - 1] * 10 + chars[i]);
        }
        dp[0] = max(dp[0], chars[i]);
    }
    dp[k - 1]
}
pub fn solve2(input: Vec<&str>) -> u64 {
    let mut res = 0;
    for s in input {
        res += largest_rel_k(s, 12);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_front() {
        let s = "98766666666";
        let exp = 98;
        let act = largest_pair1(s);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_largest_back() {
        let s = "234234234234278";
        let exp = 78;
        let act = largest_pair1(s);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_largest_front_back() {
        let s ="811111111111119";
        let exp = 89;
        let act = largest_pair1(s);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_largest_middle() {
        let s = "818181911112111";
        let exp = 92;
        let act = largest_pair1(s);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_part1() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111"
        ];
        let exp = 357;
        let act = solve1(input);
        assert_eq!(exp, act);
    }

    #[test]
    fn test_read_input() {
        let exp = vec!["987654321111111", "811111111111119", "234234234234278", "818181911112111"];
        let act = read_input("inputs/day03/input1.txt");
        assert_eq!(exp, act);
    }


    #[test]
    fn test_largest_front2() {
        let s = "987654321111111";
        let exp: u64 = 987654321111;
        let act: u64 = largest_rel_k(s, 12);
        assert_eq!(exp, act);
    }

    #[test]
    fn test_largest_back2() {
        let s = "234234234234278";
        let exp: u64 = 434234234278;
        let act: u64 = largest_rel_k(s, 12);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_largest_front_back2() {
        let s ="811111111111119";
        let exp: u64 = 811111111119;
        let act: u64 = largest_rel_k(s,12 );
        assert_eq!(exp, act);
    }
    #[test]
    fn test_largest_middle2() {
        let s = "818181911112111";
        let exp: u64 = 888911112111;
        let act: u64 = largest_rel_k(s, 12);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_part2() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111"
        ];
        let exp: u64 = 3121910778619;
        let act: u64 = solve2(input);
        assert_eq!(exp, act);
    }

}
