use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve_pt1(INPUT));
    println!("Part two: {}", solve_pt2(INPUT));
}

fn solve_pt1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::from(line).calculate_type())
        .collect();
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

fn solve_pt2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::from(line).calculate_type_with_jokers())
        .collect();
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<usize>,
    hand_type: Option<HandType>,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for i in 0..5 {
            let a = self.cards.get(i).unwrap();
            let b = other.cards.get(i).unwrap();
            if a == b {
                continue;
            }
            return a.cmp(b);
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let (cards, bid) = s.trim().split_once(" ").unwrap();
        let bid = bid.parse().unwrap();
        let cards: Vec<usize> = cards
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_digit(10).unwrap() as usize,
            })
            .collect();

        Hand {
            cards,
            hand_type: None,
            bid,
        }
    }
}

impl Hand {
    fn calculate_type(mut self) -> Hand {
        use HandType::*;

        let card_counts = card_counts(&self.cards);

        let hand_type = if card_counts.contains(&5) {
            FiveOfAKind
        } else if card_counts.contains(&4) {
            FourOfAKind
        } else if card_counts.contains(&3) && card_counts.contains(&2) {
            FullHouse
        } else if card_counts.contains(&3) {
            ThreeOfAKind
        } else if card_counts.contains(&2) && card_counts.len() == 3 {
            TwoPair
        } else if card_counts.contains(&2) {
            OnePair
        } else {
            HighCard
        };

        self.hand_type = Some(hand_type);
        self
    }

    fn calculate_type_with_jokers(mut self) -> Hand {
        use HandType::*;

        // If there are no jokers, we can just use the default so get that out the way
        if !self.cards.contains(&11) {
            return self.calculate_type();
        }

        let cards_without_jokers: Vec<usize> = self
            .cards
            .clone()
            .into_iter()
            .filter(|c| c != &11)
            .collect();

        let card_counts = card_counts(&cards_without_jokers);
        let max = card_counts.iter().max().unwrap_or(&0);
        let jokers = 5 - cards_without_jokers.len();

        let hand_type = if max + jokers == 5 {
            FiveOfAKind
        } else if max + jokers == 4 {
            FourOfAKind
        } else if max + jokers == 3 {
            // We either have:
            //   - Two pairs and one joker
            //   - One pair, two singles and one joker
            //   - Three ones and two jokers
            if card_counts.len() == 2 {
                FullHouse
            } else {
                ThreeOfAKind
            }
        } else {
            // We must have exactly one joker now and no pairs or we would have been able to to hit
            // max + jokers > 2, so we can at best do one pair.
            OnePair
        };
        self.hand_type = Some(hand_type);
        self.cards = self
            .cards
            .into_iter()
            .map(|c| if c == 11 { 1 } else { c })
            .collect();

        self
    }
}

fn card_counts(cards: &Vec<usize>) -> Vec<usize> {
    cards
        .iter()
        .fold(HashMap::new(), |mut map, x| {
            if let Some(v) = map.get_mut(&x) {
                *v += 1;
            } else {
                map.insert(x, 1);
            }
            map
        })
        .values()
        .map(|v| *v as usize)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve_pt1(EXAMPLE), 6440);
        assert_eq!(solve_pt2(EXAMPLE), 5905);
    }
}
