use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("failed to open file");
    let reader = io::BufReader::new(file);

    let mut v = Vec::new();
    for line in reader.lines() {
        v.push(line.unwrap());
    }
    v
}

/**
* n: the dial are the numbers from 0 to n - 1
* start: where dial start. e.g. 50
* rotation: list of rotation we need to take from 'start' to calculate the answer
*/
pub fn find_password(n: u32, start: u32, rotation: Vec<&str>) -> u32 {
    let mut ans = 0;
    let mut s = start;
    for rot in rotation {
        s = rotate(n, s, rot);
        if s == 0 {
            ans += 1
        }
    }
    ans
}

fn rotate(n: u32, num: u32, rot: &str) -> u32 {
    if rot.len() < 2 { panic!("{}", format!("Invalid str {rot}")); }
    let dir: char = rot.chars().next().unwrap();
    let sub = &rot[1..];
    let amt: u32 = sub.parse().expect("Integer");
    if dir == 'L' {
        return (num + n - (amt % n)) % n
    } else if dir == 'R' {
        return (num + amt) % n
    } else {
        panic!("invalid rot");
    }
}


pub fn find_password2(n: u32, start: u32, rotation: Vec<&str>) -> u32 {
    let mut ans = 0;
    let mut s: i32 = start as i32;
    let n = n as i32; 
    for rot in rotation {
        if rot.len() < 2 { panic!("{}", format!("Invalid str {rot}")); }
        let dir: char = rot.chars().next().unwrap();
        let sub = &rot[1..];
        let mut amt: i32 = sub.parse().expect("Integer");
        if dir == 'L' {
            while amt > 100 {
                ans += 1;
                amt -= 100;
            }
            if s != 0 && s - amt < 0 { 
                ans += 1;
            }
            let a = (s + n - (amt % n)) % n;
            // println!("L: {} -> {}", s - amt, a);
            s = a;
        } else if dir == 'R' {
            while amt > 100 {
                ans += 1;
                amt -= 100;
            }
            if s != 0 && s + amt > n { 
                ans += 1;
            };
            let a = (s + amt) % n;
            // println!("R: {} -> {}", s + amt, a);
            s = a;
        } else {
            panic!("invalid rot");
        }
        if s == 0 {
            ans += 1
        }
    }
    ans as u32
}


#[cfg(test)]
mod tests {
    use super::*;
    static N: u32 = 100;
    static START: u32 = 50;
    #[test]
    fn test_read_input() {
        let exp = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
        let act = read_input("inputs/day01/input1.txt");
        assert_eq!(exp, act);
    }
    #[test]
    fn test_one() {
        let rotation = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
        let exp = 3;
        let act = find_password(N, START, rotation);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_two() {
        let rotation = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
        let exp = 6;
        let act = find_password2(N, START, rotation);
        assert_eq!(exp, act);
    }
    #[test]
    fn test_rot() {
        assert_eq!(82, rotate(N, START, "L68"));
        assert_eq!(52, rotate(N, 82, "L30"));
        assert_eq!(0, rotate(N, 52, "R48"));
        assert_eq!(0, rotate(N, 52, "R48"));
    }
    #[test]
    fn test_more_cycle() {
        let rotation = vec!["L1000"];
        let exp = 10;
        let act = find_password2(N, START, rotation);
        assert_eq!(exp, act);
    }
}

