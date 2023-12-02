const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;
const EXPECT_MESSAGE: &str = "input poorly formatted";

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let game_id = parse_game_id(line);
            let round_cubes = parse_handfull(line);
            for cube in round_cubes {
                match cube {
                    Cubes::Red(amt) => {
                        if amt > MAX_RED {
                            return 0;
                        }
                    }
                    Cubes::Green(amt) => {
                        if amt > MAX_GREEN {
                            return 0;
                        }
                    }
                    Cubes::Blue(amt) => {
                        if amt > MAX_BLUE {
                            return 0;
                        }
                    }
                    Cubes::None => {
                        continue;
                    }
                }
            }
            return game_id;
        })
        .sum()
}

#[test]
fn test_d2_p1() {
    let result = part_one("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    assert_eq!(result, 8);
    let result = part_one(crate::inputs::DAY_TWO_PROMPT);
    println!("{result}");
}

fn parse_game_id(line: &str) -> usize {
    line.split(": ")
        .collect::<Vec<&str>>()
        .get(0)
        .expect(EXPECT_MESSAGE)
        .split(" ")
        .collect::<Vec<&str>>()
        .get(1)
        .expect(EXPECT_MESSAGE)
        .parse::<usize>()
        .expect(EXPECT_MESSAGE)
}

enum Cubes {
    Red(usize),
    Green(usize),
    Blue(usize),
    None,
}
fn parse_handfull(line: &str) -> Vec<Cubes> {
    line.split(": ")
        .collect::<Vec<&str>>()
        .get(1)
        .expect(EXPECT_MESSAGE)
        .split("; ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|handfull| {
            handfull
                .split(", ")
                .map(|cubes| {
                    let mut num: usize = 0;
                    let mut color: &str = "";
                    cubes.split(" ").enumerate().for_each(|(idx, val)| {
                        if idx == 0 {
                            num = val.parse::<usize>().expect(EXPECT_MESSAGE);
                        } else if idx == 1 {
                            color = val;
                        }
                    });
                    match color {
                        "red" => return Cubes::Red(num.clone()),
                        "green" => return Cubes::Green(num.clone()),
                        "blue" => return Cubes::Blue(num.clone()),
                        _ => return Cubes::None,
                    }
                })
                .collect::<Vec<Cubes>>()
        })
        .collect::<Vec<Vec<Cubes>>>()
        .into_iter()
        .flat_map(|v| v.into_iter().collect::<Vec<Cubes>>())
        .collect::<Vec<Cubes>>()
}
