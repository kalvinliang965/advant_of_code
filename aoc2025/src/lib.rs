mod day01;
mod day02;


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
        let exp: u128 = 1227775554;
        let act: u128 = run_day02("inputs/day02/input1.txt");
        assert_eq!(exp, act);
    }
}
