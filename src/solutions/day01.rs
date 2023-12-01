pub fn solve(problem: &str) -> usize {
    let lines = problem.split("\n");
    lines
        .map(|l| {
            let mut chars = l.chars();
            (
                chars.find(|c| c.is_digit(10)).unwrap(),
                chars.rfind(|c| c.is_digit(10)).unwrap(),
            )
        })
        .map(|(f, s)| f.to_digit(10).unwrap() * 10 + s.to_digit(10).unwrap())
        .sum::<u32>() as usize
}
