use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub enum FileError {
    OpenFailed,
    ReadFailed,
    WriteFailed,
    InvalidFormat,
}


pub fn read_input(filepath: &str) -> Result<Vec<Vec<char>>, FileError> {
    let file = File::open(filepath).map_err(|_| FileError::OpenFailed)?;
    let reader = BufReader::new(file);
    let mut res: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|_| FileError::ReadFailed)?;
        res.push(line.chars().collect());
    }
    Ok(res)
}

pub fn solve1(v: &mut Vec<Vec<char>>) -> u64 {
    let mut res: u64 = 0;
    assert!(v.len() > 0);
    let m = v.len();
    let n = v[0].len();
    for i in 0..m {
        for j in 0..n {
            match v[i][j] {
                'S' | '|' => {
                    if i + 1 < m {
                        if v[i + 1][j] == '^' {
                            if i + 2  < m {
                                if j as isize - 1 >= 0 {
                                    v[i + 2][j - 1] = '|';
                                }
                                if j + 1 < n { 
                                    v[i + 2][j + 1] = '|';
                                }
                            }
                            res += 1;
                        } else {
                            v[i + 1][j] = '|';
                        }
                    }
                },
                _ => {}
            }
        }
    }
    res
}

pub fn solve2(v: &mut Vec<Vec<char>>) -> u64 {
    assert!(v.len() > 0);
    let m = v.len();
    let n = v[0].len();
    let mut dp = vec![vec![0; n]; m];
    for j in 0..n {
        dp[m - 1][j] = 1;
    }
    for i in (0..(m - 1)).rev() {
        for j in 0..n {
            match v[i][j] {
                'S' => {
                    if i + 1 < m {
                        return dp[i + 1][j];
                    }
                    return 0;
                },
                '^' => {
                    if i + 1 < m {
                        if j as isize - 1 >= 0 {
                            dp[i][j] += dp[i + 1][j - 1];
                        }
                        if j + 1 < n { 
                            dp[i][j] += dp[i + 1][j + 1];
                        }
                    }

                },
                _ => {
                    if i + 1 < m {
                        dp[i][j] += dp[i + 1][j];
                    }
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let exp: Vec<Vec<char>> = vec![
            ".......S.......".chars().collect(),
            "...............".chars().collect(),
            ".......^.......".chars().collect(),
            "...............".chars().collect(),
            "......^.^......".chars().collect(),
            "...............".chars().collect(),
            ".....^.^.^.....".chars().collect(),
            "...............".chars().collect(),
            "....^.^...^....".chars().collect(),
            "...............".chars().collect(),
            "...^.^...^.^...".chars().collect(),
            "...............".chars().collect(),
            "..^...^.....^..".chars().collect(),
            "...............".chars().collect(),
            ".^.^.^.^.^...^.".chars().collect(),
            "...............".chars().collect()
        ];
        let act = read_input("inputs/day07/input1.txt").unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn test_solve2() {
        let mut v: Vec<Vec<char>> = vec![
            ".......S.......".chars().collect(),
            "...............".chars().collect(),
            ".......^.......".chars().collect(),
            "...............".chars().collect(),
            "......^.^......".chars().collect(),
            "...............".chars().collect(),
            ".....^.^.^.....".chars().collect(),
            "...............".chars().collect(),
            "....^.^...^....".chars().collect(),
            "...............".chars().collect(),
            "...^.^...^.^...".chars().collect(),
            "...............".chars().collect(),
            "..^...^.....^..".chars().collect(),
            "...............".chars().collect(),
            ".^.^.^.^.^...^.".chars().collect(),
            "...............".chars().collect()
        ];
        let exp: u64 = 40;
        let act = solve2(&mut v);
        assert_eq!(exp, act);

    }
}
