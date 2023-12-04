#[derive(Debug)]
struct Card {
    pub winning_numbers: Vec<usize>,
    pub owned_numbers: Vec<usize>,
    pub card_id: usize,
    pub card_copies: usize,
}
impl Card {
    fn new(line: &str) -> Self {
        let mut winning_numbers: Vec<usize> = Vec::new();
        let mut owned_numbers: Vec<usize> = Vec::new();
        let mut card_id: usize = 0;
        line.split(": ").enumerate().for_each(|(idx, val)| {
            if idx == 0 {
                val.split(" ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|val| match val.parse::<usize>() {
                        Ok(num) => card_id = num,
                        Err(_) => {}
                    });
            } else if idx == 1 {
                val.split("|")
                    .collect::<Vec<&str>>()
                    .iter()
                    .enumerate()
                    .for_each(|(idx, val)| {
                        if idx == 0 {
                            val.split(" ").for_each(|num| match num.parse::<usize>() {
                                Ok(num) => winning_numbers.push(num),
                                Err(_) => {}
                            });
                        } else if idx == 1 {
                            val.split(" ").for_each(|num| match num.parse::<usize>() {
                                Ok(num) => owned_numbers.push(num),
                                Err(_) => {}
                            });
                        }
                    });
            }
        });

        return Card {
            winning_numbers,
            owned_numbers,
            card_id,
            card_copies: 0,
        };
    }
}

pub fn part_one(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| Card::new(line))
        .collect::<Vec<Card>>();
    cards
        .iter()
        .map(|card| {
            let mut total = 0;
            card.owned_numbers.iter().for_each(|num| {
                if card.winning_numbers.contains(num) {
                    if total == 0 {
                        total += 1
                    } else {
                        total = total * 2;
                    }
                }
            });
            total
        })
        .sum()
}

#[test]
fn test_d4_p1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let res = part_one(input);
    assert_eq!(res, 13);
    let res = part_one(crate::inputs::DAY_FOUR_PROMPT);
    println!("{res}");
}

pub fn part_two(input: &str) -> usize {
    let mut cards = input
        .lines()
        .map(|line| Card::new(line))
        .collect::<Vec<Card>>();

    for card_idx in 0..cards.len() {
        let mut wins = 0;
        for num in cards[card_idx].owned_numbers.iter() {
            if cards[card_idx].winning_numbers.contains(num) {
                wins += 1;
            }
        }
        for copy_idx in card_idx + 1..=card_idx + wins {
            cards[copy_idx].card_copies += cards[card_idx].card_copies + 1;
        }
    }
    let copy_sum: usize = cards.iter().map(|card| card.card_copies).sum();
    return copy_sum + cards.len();
}

#[test]
fn test_d4_p2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let res = part_two(input);
    assert_eq!(res, 30);
    let res = part_two(crate::inputs::DAY_FOUR_PROMPT);
    println!("{res}");
}
