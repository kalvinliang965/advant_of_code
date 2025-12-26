use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::{max, min};
use std::collections::{HashSet};

#[derive(Debug)]
pub enum FileError {
    InvalidFormat,
    OpenFailed,
    ReadFailed,
    WriteFaield,
}

pub fn read_input(filepath: &str) -> Result<Vec<(u64, u64)>, FileError> {
    let file = File::open(filepath).map_err(|_| FileError::OpenFailed)?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line = line.map_err(|_| FileError::ReadFailed)?;
            let nums: Vec<u64> = line
                .split(",")
                .map(|s| s.parse().map_err(|_| FileError::InvalidFormat))
                .collect::<Result<_, _>>()?;
            Ok((nums[0], nums[1]))

        })
        .collect()
}

pub fn solve1(v: & Vec<(u64, u64)>) -> u64 {
    let mut res = 0;
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            let (x1, y1) = v[i];
            let (x2, y2) = v[j];
            let area = ((x2 as i64 - x1 as i64).abs() as u64 + 1u64) * ((y2 as i64 - y1 as i64).abs() as u64 + 1u64);
            res = max(res, area);
        }
    }
    res
}

fn print_2d_vector(v: &Vec<Vec<u32>>) {
    for row in v {
        for e in row {
            print!("{}, ", e);
        }
        println!("");
    }
    println!("");
}

/**
* points - pair of points in format (y, x)
*/
fn color_mat(
    points: &Vec<(u64, u64)>,
    mat: &mut Vec<Vec<u32>>,
    start_x: &u64,
    start_y: &u64,
) {
    let m = mat.len();
    let n = mat[0].len();

    /* =========================
       1. Mark given points (O(k))
       ========================= */
    for &(y, x) in points {
        let i = (x - start_x) as usize;
        let j = (y - start_y) as usize;
        mat[i][j] = 1;
    }

    /* =========================
       2. Scan rows (O(mn))
       ========================= */
    for i in 0..m {
        let mut l = 0;
        let mut r = n - 1;

        while l < n && mat[i][l] == 0 {
            l += 1;
        }
        while r > 0 && mat[i][r] == 0 {
            r -= 1;
        }

        if l < r {
            for col in (l + 1)..r {
                mat[i][col] = 2;
            }
        }
    }

    /* =========================
       3. Scan columns (O(mn))
       ========================= */
    for j in 0..n {
        let mut l = 0;
        let mut r = m - 1;

        while l < m && mat[l][j] == 0 {
            l += 1;
        }
        while r > 0 && mat[r][j] == 0 {
            r -= 1;
        }

        if l < r {
            for row in (l + 1)..r {
                mat[row][j] = 2;
            }
        }
    }
}
/**
* v - pair of points in format (y, x)
*/
pub fn solve2_old(v: & Vec<(u64, u64)>) -> u64 {
    let mut res = 0;
    let mut start_x = u64::MAX;
    let mut start_y = u64::MAX;
    let mut end_x = u64::MIN;
    let mut end_y = u64::MIN;
    for (y, x) in v {
        start_x = min(*x, start_x);
        end_x = max(*x, end_x);
        start_y = min(*y, start_y);
        end_y = max(*y, end_y);
    }
    let m = (end_x - start_x + 1) as usize;
    let n = (end_y - start_y + 1) as usize;
    let mut mat = vec![vec![0 as u32; n]; m];
    color_mat(&v, &mut mat, &start_x, &start_y);
    for i in 0..v.len() {
        let (y1, x1) = v[i];
        for j in (i + 1)..v.len() {
            let (y2, x2) = v[j];
            // get the other two points
            let p1 = (x1, y2);
            let p2 = (x2, y1);
            if mat[(p1.0 - start_x) as usize][(p1.1 - start_y) as usize] != 0 && mat[(p2.0 - start_x) as usize][(p2.1 - start_y) as usize] != 0 {
                let area = ((x2 as i64 - x1 as i64).abs() as u64 + 1u64) * ((y2 as i64 - y1 as i64).abs() as u64 + 1u64);
                res = max(res, area);
            }
        }
    }
    res
}

type Point = (u64, u64);
pub fn solve2(red: &Vec<Point>) -> u64 {
    let n = red.len();
    let mut lines: Vec<(Point, Point)> = Vec::new();
    for i in 0..n {
        let (x1, y1) = red[i];
        let (x2, y2) = red[(i + 1) % n]; 
        lines.push(((x1, y1), (x2, y2)));
    }
    let mut cand: Vec<(u64, (u64, u64), (u64, u64))>  = Vec::new();
    for i in 0..n {
        let (x1, y1) = red[i];
        for j in (i + 1)..n {
            let (x2, y2) = red[j];
            let area = ((x2 as i64 - x1 as i64).abs() as u64 + 1u64) * ((y2 as i64 - y1 as i64).abs() as u64 + 1u64);
            cand.push(( area, (x1, y1), (x2, y2) ))
        }
    }
    cand.sort();
    cand.reverse();
    for (area, (x1, y1), (x2, y2)) in cand {
        let minx = min(x1, x2);
        let maxx = max(x1, x2);
        let miny = min(y1, y2);
        let maxy = max(y1, y2);
         let ok = lines.iter().all(|(line_start, line_end)| {
            let lsx = line_start.0;
            let lsy = line_start.1;
            let lex = line_end.0;
            let ley = line_end.1;
            let left_of_rect =
                maxx <= min(lsx, lex);
            let right_of_rect =
                minx >= max(lsx, lex);
            let above =
                maxy <= min(lsy, ley);
            let below =
                miny >= max(lsy, ley);
            left_of_rect || right_of_rect || above || below
        });
        if ok {
            return area;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let act = read_input("inputs/day09/input1.txt").unwrap();
        let exp = vec![
            (7,1),
            (11,1),
            (11,7),
            (9,7),
            (9,5),
            (2,5),
            (2,3),
            (7,3)
        ];
        assert_eq!(exp, act);
    }
    #[test]
    fn test_solve1() {
        let mut v = vec![
            (7,1),
            (11,1),
            (11,7),
            (9,7),
            (9,5),
            (2,5),
            (2,3),
            (7,3)
        ];
        let act = solve1(& v);
        let exp = 50;
        assert_eq!(exp, act);
    }
    #[test]
    fn test_solve2() {
        let mut v = vec![
            (7,1),
            (11,1),
            (11,7),
            (9,7),
            (9,5),
            (2,5),
            (2,3),
            (7,3)
        ];
        let act = solve2(& v);
        let exp = 24;
        assert_eq!(exp, act);
    }
}
