mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn run_day01(filepath: &str) -> u32 {
    let v = day01::read_input(filepath);
    const START: u32 = 50;
    const N: u32 = 100;
    day01::find_password2(N, START, v.iter().map(|s| s.as_str()).collect())
}

pub fn run_day02(filepath: &str) -> u128 {
    let v = day02::read_input(filepath);
    day02::count_invalid_ids2(v)
}

pub fn run_day03(filepath: &str) -> u64 {
    let v = day03::read_input(filepath);
    day03::solve2(v.iter().map(|s| s.as_str()).collect())
}

pub fn run_day04(filepath:& str) -> u64 {
    let mut v = day04::read_input(filepath);
    day04::solve2(&mut v)
}

pub fn run_day05(filepath: &str) -> u64 {
    let (v, _) = day05::read_input(filepath);
    day05::solve2(&v)
}

pub fn run_day06(filepath: &str) -> u64 {
    let map = day06::read_input2(filepath).expect("failed to parse file");
    day06::solve(&map)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day01_sample() {
        let exp = 6;
        let act = run_day01("inputs/day01/input1.txt");
        assert_eq!(exp, act);
    }
    #[test]
    fn test_day02_sample() {
        let exp: u128 = 4174379265;
        let act: u128 = run_day02("inputs/day02/input1.txt");
        assert_eq!(exp, act);
    }
    #[test]
    fn test_day03_sample() {
        let exp = 3121910778619;
        let act = run_day03("inputs/day03/input1.txt");
        assert_eq!(exp, act);
    }

    #[test]
    fn test_day04_sample() {
        let exp = 43;
        let act = run_day04("inputs/day04/input1.txt");
        assert_eq!(exp, act);
    }
    #[test]
    fn test_day05_sample() {
        let exp = 14;
        let act = run_day05("inputs/day05/input1.txt");
        assert_eq!(exp, act);
    }
    #[test]
    fn test_day06_sample() {
        let exp = 3263827;
        let act = run_day06("inputs/day06/input1.txt");
        assert_eq!(exp, act);
    }
}
