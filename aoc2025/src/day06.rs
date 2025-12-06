use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseOpError;

#[derive(Debug)]
pub enum InputError {
    ParseOp,
    MissingOp,
    SparseFile,
    InvalidNumber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn new(c: char) -> Result<Self, ParseOpError> {
        match c {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '/' => Ok(Self::Div),
            _ => Err(ParseOpError),
        }
    }
}

pub fn read_input2(filepath: &str) -> Result<HashMap<Op,Vec<Vec<u64>>>, InputError>  {
    let file = File::open(filepath).expect("failed to open day06 input file");
    let reader = BufReader::new(file);
    let mut res: HashMap<Op, Vec<Vec<u64>>> = HashMap::new();
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let m = lines.len();    // there should be m number in each col group
    let n = lines[0].len(); // there should be n column
    
    let mut cols: Vec<i128> = vec![-1; n];
    let mut ops: Vec<Op>   = Vec::new();
    
    let mut col = 0;
    for (i, line) in lines.into_iter().enumerate(){
        // we expect last row to be the operator 
        if i == m - 1 {
            for (j, token) in line.split_whitespace().enumerate() {
                let c = token.chars().next().ok_or(InputError::ParseOp)?;
                let op = Op::new(c).map_err(|_|InputError::ParseOp)?;
                ops.push(op);
            }
        } else {
            col = 0;
            let mut prev = '@';
            for (j, token) in line.chars().enumerate() {
                if token == ' ' {
                    if prev != ' ' {
                        col +=1 
                    }
                } else {
                    let t = token.to_digit(10).ok_or(InputError::InvalidNumber)? as i128;
                    if cols[j] == -1 {
                        cols[j] = t;
                    } else {
                        cols[j] = cols[j]* 10 + t;
                    }
                }
                prev = token;
            }
        }
    }
    
    let mut op_index = 0;
    let mut prev = -1;
    for c in cols {
        if c == -1{
            if prev != -1 {
                op_index += 1;
            }
        } else {
            let op = ops.get(op_index).ok_or(InputError::ParseOp)?;
            let mut col_groups = res.entry(*op).or_insert(Vec::new());
            if prev == -1 {
                col_groups.push(Vec::new());
            }
            col_groups.last_mut().unwrap().push(c as u64);
        }
        prev = c;
    }
    Ok(res)
}

pub fn read_input1(filepath: &str) -> Result<HashMap<Op, Vec<Vec<u64>>>, InputError>  {
    let file = File::open(filepath).expect("failed to open day06 input file");
    let reader = BufReader::new(file);
    let mut res: HashMap<Op, Vec<Vec<u64>>> = HashMap::new();
    let mut op_map: HashMap<usize, Op> = HashMap::new(); 
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for (i, line) in lines.into_iter().rev().enumerate(){
        // we expect last row to be the operator 
        if i == 0 {
            for (j, token) in line.split_whitespace().enumerate() {
                let c = token.chars().next().ok_or(InputError::ParseOp)?;
                let op = Op::new(c).map_err(|_|InputError::ParseOp)?;
                op_map.insert(j, op);
            }
        } else {
            for (j, token) in line.split_whitespace().enumerate() {
                let op = op_map.get(&j).ok_or(InputError::SparseFile)?;
                let val: u64 = token.parse().map_err(|_| InputError::InvalidNumber)?;
                let col_groups = res.entry(*op).or_insert(Vec::new());
                if col_groups.len() <= j {
                    col_groups.resize_with(op_map.len(), || Vec::new());
                }
                col_groups[j].push(val);
            }
        }
    }
    for groups in res.values_mut() {
        groups.retain(|g| !g.is_empty());
    }
    Ok(res)
}

pub fn solve(map: &HashMap<Op, Vec<Vec<u64>>>) -> u64 {
    let mut res: u64 = 0;
    for (key, val) in map {
        match key {
            Op::Add => {
                for v in val {
                    let mut t = 0;
                    for e in v {
                        t += e;
                    }
                    res += t;
                }
            },
            Op::Mul => {
                for v in val {
                    let mut t = 1;
                    let mut b = false;
                    for e in v {
                        b = true;
                        t *= e;
                    }
                    if b {
                        res += t;
                    }
                }
            }
            _ => {}
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input1() {
        let map = read_input1("inputs/day06/input1.txt").expect("parse fail");
        assert_eq!(map[&Op::Add], vec![
            vec![98,64,328 ],
            vec![314,23,64 ]
        ]);
        assert_eq!(map[&Op::Mul], vec![
            vec![6, 45, 123],
            vec![215, 387, 51]
        ]);
    }
    #[test]
    fn test_read_input2() {
        let map = read_input2("inputs/day06/input1.txt").expect("parse fail");
        let exp = vec![
            vec![369, 248, 8],
            vec![623, 431, 4]
        ];
        assert_eq!(exp, map[&Op::Add]);
        let exp = vec![
            vec![1, 24, 356],
            vec![32, 581, 175]
        ];
        assert_eq!(exp, map[&Op::Mul]);
    }
}



