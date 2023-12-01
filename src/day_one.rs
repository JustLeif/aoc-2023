use crate::inputs::day_one_prompt;

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
            let mut res = chars
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx == &0 || idx == &(chars.len() - 1))
                .map(|(_, val)| *val)
                .collect::<String>();
            if res.len() == 1 {
                res.push(res.chars().nth(0).unwrap());
            }
            res.parse::<usize>().unwrap()
        })
        .sum()
}

#[test]
fn test_part_one() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    assert_eq!(part_one(input), 142);
    let res = part_one(day_one_prompt());
    println!("{res}");
}

#[test]
fn test_part_two() {
    let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    assert_eq!(part_two(input), 281);
    let res = part_two(day_one_prompt());
    println!("{res}");
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut res: Vec<char> = Vec::new();
            for (idx, val) in line.chars().enumerate() {
                if val.is_numeric() {
                    res.push(val);
                } else if idx + 3 <= line.len() && &line[idx..idx + 3] == "one" {
                    res.push('1');
                } else if idx + 3 <= line.len() && &line[idx..idx + 3] == "two" {
                    res.push('2');
                } else if idx + 5 <= line.len() && &line[idx..idx + 5] == "three" {
                    res.push('3');
                } else if idx + 4 <= line.len() && &line[idx..idx + 4] == "four" {
                    res.push('4');
                } else if idx + 4 <= line.len() && &line[idx..idx + 4] == "five" {
                    res.push('5');
                } else if idx + 3 <= line.len() && &line[idx..idx + 3] == "six" {
                    res.push('6');
                } else if idx + 5 <= line.len() && &line[idx..idx + 5] == "seven" {
                    res.push('7');
                } else if idx + 5 <= line.len() && &line[idx..idx + 5] == "eight" {
                    res.push('8');
                } else if idx + 4 <= line.len() && &line[idx..idx + 4] == "nine" {
                    res.push('9');
                }
            }
            res = res
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx == &0 || idx == &(res.len() - 1))
                .map(|(_, val)| *val)
                .collect();
            if res.len() == 1 {
                res.push(res[0].clone());
            }
            let res = res.iter().collect::<String>();
            res.parse::<usize>().unwrap()
        })
        .sum()
}
