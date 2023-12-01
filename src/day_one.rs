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

#[cfg(test)]
mod test {
    use crate::{day_one::part_one, inputs::day_one_part_one_prompt};

    #[test]
    fn test_part_one() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part_one(input), 142);
        let res = part_one(day_one_part_one_prompt());
        println!("{res}");
    }
}
