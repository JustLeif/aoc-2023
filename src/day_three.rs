pub fn part_one(input: &str) -> usize {
    let char_matrix = parse_input_to_char_matrix(input);
    let mut total: usize = 0;
    char_matrix.iter().enumerate().for_each(|(y_idx, line)| {
        line.iter().enumerate().for_each(|(x_idx, c)| {
            if is_symbol(c) == true {
                let res = adjacent_numbers(x_idx, y_idx, &char_matrix);
                res.iter().for_each(|num| {
                    total += num;
                });
            }
        })
    });
    total
}

#[test]
fn test_d3_p1() {
    let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    assert_eq!(part_one(input), 4361);
    let res = part_one(crate::inputs::DAY_THREE_PROMPT);
    println!("{res}");
}

pub fn part_two(input: &str) -> usize {
    let char_matrix = parse_input_to_char_matrix(input);
    let mut total: usize = 0;
    char_matrix.iter().enumerate().for_each(|(y_idx, line)| {
        line.iter().enumerate().for_each(|(x_idx, c)| {
            if is_symbol(c) == true {
                let res = adjacent_numbers(x_idx, y_idx, &char_matrix);
                if c == &'*' && res.len() == 2 {
                    total += res[0] * res[1];
                }
            }
        })
    });
    total
}

#[test]
fn test_d3_p2() {
    let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    assert_eq!(part_two(input), 467835);
    let res = part_two(crate::inputs::DAY_THREE_PROMPT);
    println!("{res}");
}

fn is_symbol(c: &char) -> bool {
    if c.is_numeric() {
        return false;
    } else if c == &'.' {
        return false;
    } else {
        return true;
    }
}

fn parse_input_to_char_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

#[derive(Debug, Clone)]
struct Point {
    pub x: usize,
    pub y: usize,
}

fn adjacent_numbers(x: usize, y: usize, char_matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut adjacent: Vec<Point> = Vec::new();

    // Check top left position
    if y > 0 && x > 0 && char_matrix[y - 1][x - 1].is_numeric() {
        adjacent.push(Point { x: x - 1, y: y - 1 });
    }

    // Check top position
    if y > 0 && char_matrix[y - 1][x].is_numeric() {
        adjacent.push(Point { x, y: y - 1 });
    }

    // Check top right position
    if y > 0 && x + 1 < char_matrix[y - 1].len() && char_matrix[y - 1][x + 1].is_numeric() {
        adjacent.push(Point { x: x + 1, y: y - 1 });
    }

    // Check left position
    if x > 0 && char_matrix[y][x - 1].is_numeric() {
        adjacent.push(Point { x: x - 1, y });
    }

    // Check right position
    if x + 1 < char_matrix[y].len() && char_matrix[y][x + 1].is_numeric() {
        adjacent.push(Point { x: x + 1, y });
    }

    // Check bottom left position
    if y + 1 < char_matrix.len() && x > 0 && char_matrix[y + 1][x - 1].is_numeric() {
        adjacent.push(Point { x: x - 1, y: y + 1 });
    }

    // Check bottom position
    if y + 1 < char_matrix.len() && char_matrix[y + 1][x].is_numeric() {
        adjacent.push(Point { x, y: y + 1 });
    }

    // Check bottom right position
    if y + 1 < char_matrix.len()
        && x + 1 < char_matrix[y].len()
        && char_matrix[y + 1][x + 1].is_numeric()
    {
        adjacent.push(Point { x: x + 1, y: y + 1 });
    }

    // Determine the starting index of all points so we can return the full number.
    adjacent.iter_mut().for_each(|p| {
        while p.x > 0 && char_matrix[p.y][p.x - 1].is_numeric() {
            p.x -= 1;
        }
    });
    // Remove duplicates
    adjacent.dedup_by(|a, b| a.x == b.x && a.y == b.y);

    // Convert into a vec of integers
    adjacent
        .iter_mut()
        .map(|p| {
            let mut amt: usize = 0;
            while p.x < char_matrix[p.y].len() && char_matrix[p.y][p.x].is_numeric() {
                amt = (amt * 10) + char_matrix[p.y][p.x].to_digit(10).unwrap() as usize;
                p.x += 1;
            }
            amt
        })
        .collect::<Vec<usize>>()
}
