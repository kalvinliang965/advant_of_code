use std::fs::File;
use std::io::{self, BufRead};


pub fn read_input(filepath: &str) -> Vec<Vec<char>> {
    let file = File::open(filepath).expect("failed to open day04 file");
    let reader = io::BufReader::new(file);
    let mut v = Vec::new();
    for line in reader.lines() {
        v.push(line.unwrap().chars().collect());
    }
    v
}

static DIRS: [(isize, isize); 8] = [
    (-1, 0), // top
    (1, 0), // bot 
    (0, -1), // left
    (0, 1), // right
    (-1, -1), // top left
    (-1, 1), // top-right
    (1, -1), // bot-left
    (1, 1) // bot-right
];

/**************************** part1 ****************/

pub fn solve1(v: &mut Vec<Vec<char>>) -> u64 {
    let m = v.len();
    assert!(m > 1, "v should has at least one row");
    let n = v[0].len();
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if v[i as usize][j as usize] != '@' {
                continue;
            }
            let mut cnt = 0;
            for (dx, dy) in DIRS {
                let ni = i as isize + dx;
                let nj = j as isize + dy;
                if ni >= 0 && ni < m as isize && nj >= 0 && nj < n as isize {
                    let c = v[ni as usize][nj as usize];
                    if c == '@' || c == 'x' {
                        cnt += 1;
                    }
                }
            }
            if cnt < 4 {
                v[i as usize][j as usize] = 'x';
                res += 1;
            }
        }
    }
    for i in 0..m {
        for j in 0..n {
            if v[i as usize][j as usize] == 'x' {
                v[i as usize][j as usize] = '.';
            }
        }
    }
    res 
}

pub fn solve2(v: &mut Vec<Vec<char>>) -> u64 {
    let mut res = 0;
    loop {
        let t = solve1(v);
        if t == 0 { break; }
        res += t;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let mut v: Vec<Vec<char>> = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@."
        ].iter().map(|row| row.chars().collect()).collect();
        let exp = 13 as u64;
        let act = solve1(&mut v) as u64;
        assert_eq!(exp, act);
    }

    #[test]
    fn test_part2() {
        let mut v: Vec<Vec<char>> = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@."
        ].iter().map(|row| row.chars().collect()).collect();
        let exp = 43 as u64;
        let act = solve2(&mut v) as u64;
        assert_eq!(exp, act);
    }
}
