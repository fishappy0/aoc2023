use std::collections::HashMap;
pub fn solve_p1(problem: &str) -> usize {
    let lines = problem.split("\n");
    let definition = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    lines
        .map(|l| {
            let game_n_id = l.split(": ").collect::<Vec<&str>>();
            let game_id = game_n_id[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();

            let is_game_legal = game_n_id[1]
                .split(";")
                .collect::<Vec<&str>>()
                .into_iter()
                .find(|&x| {
                    let items = x.split(",").collect::<Vec<&str>>();
                    match items.iter().find(|&i| {
                        let current_item = i.trim().split(" ").collect::<Vec<&str>>();
                        let num = current_item[0].parse::<usize>().unwrap();
                        let color = current_item[1];
                        match color {
                            "red" => num > definition[color],
                            "green" => num > definition[color],
                            "blue" => num > definition[color],
                            _ => {
                                panic!(
                                    "Unknown input, input {color}, game:{game_id}, item:{i}, x:{x}"
                                )
                            }
                        }
                    }) {
                        Some(_) => true,
                        None => false,
                    }
                })
                .unwrap_or("No illegal items found")
                .contains("No illegal items found");
            match &is_game_legal {
                true => game_id,
                false => 0,
            }
        })
        .sum()
}
