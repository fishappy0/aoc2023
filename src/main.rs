mod solutions;
use std::{fs, io::Write};

fn main() {
    let day01_prob = fs::read_to_string("./inputs/day01.txt").unwrap();
    let day01_sol = solutions::day01::solve(&day01_prob);
    let mut day01_out = fs::File::create("./outputs/day01.txt").unwrap();
    day01_out.write(day01_sol.to_string().as_bytes()).unwrap();
}
