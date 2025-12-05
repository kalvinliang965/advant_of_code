use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input(filepath: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = File::open(filepath).expect("unable to open day05 input file");
    let reader = io::BufReader::new(file);
    let mut intervals = Vec::new();
    let mut nums = Vec::new();
    let mut current_section = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            current_section += 1;
            continue;
        }
        match current_section {
            0 => {
                let v = line.split("-").map(|s| s.parse::<u64>().expect("section one should contain tuple integer")).collect::<Vec<u64>>();
                assert!(v.len() >= 2, "section 1 contain invalid number of pair");
                intervals.push((v[0], v[1]));
            },
            1 => {
                let num: u64 = line.parse().expect("section 2 suppose to contain integer");
                nums.push(num);
            },
            _ => panic!("Invalid input format: contain mutiple section")
        }
    }

    (intervals, nums)
}

fn merge_intervals(intervals: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut intervals = intervals.clone();
    intervals.sort();
    let mut res = Vec::new();
    for (a, b) in intervals {
        if res.is_empty() {
            res.push((a, b));
        } else {
            let mut last = res.last_mut().unwrap();
            if a > last.1 {
                res.push((a, b));
            } else {
                last.1 = max(last.1, b as u64);
            }
        }
    }

    res
}

fn check1(n: usize, intervals: &Vec<(u64, u64)>, num: u64) -> bool {
    let mut l = 0;
    let mut r = n as isize - 1 ;
    while l <= r {
        let m = l as usize + (r as usize - l as usize) / 2 as usize;
        let (a, b) = intervals[m];
        if num >= a && num <= b {
            return true;
        } else if num < a {
            r = m as isize - 1;
        } else {
            l = m as isize + 1;
        }
    }
    false
}
pub fn solve1(good: &Vec<(u64, u64)>, nums: &Vec<u64>) -> u64 {
    let mut res: u64 = 0;
    // after merging, the intervals are sorted. 
    let intervals = merge_intervals(good);
    let n = intervals.len();
    for num in nums {
        if check1(n, &intervals, *num) {
            res += 1;
        }
    }
    res
}

pub fn solve2(good: &Vec<(u64, u64)>) -> u64 {
    let mut res: u64 = 0;
    // after merging, the intervals are sorted. 
    let intervals = merge_intervals(good);
    for (a, b) in intervals {
        res += b - a + 1;
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_input() {
        let (v, w) = read_input("inputs/day05/input1.txt");
        assert_eq!(vec![(3, 5), (10, 14), (16,20), (12,18)], v);
        assert_eq!(vec![1,5,8,11,17,32], w);
    }
    #[test]
    fn test_mergee_intervals() {
        let v: Vec<(u64, u64)> = vec![(3,5), (10,14), (16, 20), (12,18)];
        assert_eq!(vec![(3,5), (10, 20)], merge_intervals(&v));
    }

    #[test]
    fn test_solve1() {
        let v: Vec<(u64, u64)> = vec![(3,5), (10,14), (16, 20), (12,18)];
        let w: Vec<u64> = vec![1,5,8,11,17,32];
        assert_eq!(3, solve1(&v, &w));
    }

    #[test]
    fn test_solve2() {
        let v: Vec<(u64, u64)> = vec![(3,5), (10,14), (16, 20), (12,18)];
        assert_eq!(14, solve2(&v));
    }
    #[test]
    fn test_check1() {
        let v: Vec<(u64, u64)> =  vec![(3,5), (10, 20)];
        let n = 2;
        assert!(check1(n, &v, 4));
        assert!(check1(n, &v, 12));
        assert!(!check1(n, &v, 7));
    }
}
