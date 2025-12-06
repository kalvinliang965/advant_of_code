use aoc2025::run_day01;
use aoc2025::run_day02;
use aoc2025::run_day03;
use aoc2025::run_day04;
use aoc2025::run_day05;
use aoc2025::run_day06;

fn main() {
    println!("day01: password: {}", run_day01("inputs/day01/input2.txt"));
    println!("day01: invalid ids: {}", run_day02("inputs/day02/input2.txt"));
    println!("day03: max joltage possible: {}", run_day03("inputs/day03/input2.txt"));
    println!("day04: num rolls of paper: {}", run_day04("inputs/day04/input2.txt"));
    println!("day05: number of available ingredient IDs: {}", run_day05("inputs/day05/input2.txt"));
    println!("day06: grand total: {}", run_day06("inputs/day06/input2.txt"));
}
