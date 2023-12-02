mod solutions;
use std::{fs, io::Write};

fn main() {
    let day01_prob = fs::read_to_string("./inputs/day01.txt").unwrap();
    let day01_sol = solutions::day01::solve(&day01_prob);
    let mut day01_out = fs::File::create("./outputs/day01.txt").unwrap();
    day01_out.write(day01_sol.to_string().as_bytes()).unwrap();

    let day02_prob = fs::read_to_string("./inputs/day02.txt").unwrap();
    let day02_p1_sol = solutions::day02::solve_p1(&day02_prob);
    let mut day02_out = fs::File::create("./outputs/day02.txt").unwrap();
    day02_out
        .write(format!("Part 1: {:?}, Part 2: ", day02_p1_sol.to_string()).as_bytes())
        .unwrap();
}
