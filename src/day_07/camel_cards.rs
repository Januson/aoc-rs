use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from_char(char: char) -> Self {
        match char {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown suite")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Round {
    hand: Hand,
    bid: u32,
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.trim().split_once(' ').unwrap();
        let hand = hand.chars()
            .map(|x| Card::from_char(x))
            .collect::<Vec<Card>>();
        let bid = bid.parse::<u32>().unwrap();

        Ok(
            Round {
                hand: Hand(hand),
                bid,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand(Vec<Card>);

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.hand_type().cmp(&self.hand_type()) {
            Ordering::Equal => {
                let order = self.0.iter().zip(other.0.iter())
                    .filter(|(a, b)| a != b)
                    .find_map(|(a, b)| {
                        match a.cmp(&b) {
                            Ordering::Equal => None,
                            order => Some(order.reverse()),
                        }
                    }).unwrap();
                Some(order)
            }
            hand => Some(hand)
        }
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let frequencies = frequencies(&self.0);
        let mut values: Vec<i32> = frequencies.values().map(|x| *x).collect();
        values.sort_by(|a, b| b.cmp(a));

        match values[..] {
            [5] => HandType::FiveKind,
            [4, 1] => HandType::FourKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Should not happen!")
        }
    }
}

fn frequencies<T: Copy + Hash + Eq>(list: &Vec<T>) -> HashMap<T, i32> {
    list.iter()
        .copied()
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val)
                .and_modify(|frq| *frq += 1)
                .or_insert(1);
            map
        })
}

fn ordered_rounds(input: &Vec<&str>) -> Vec<Round> {
    let mut rounds: Vec<Round> = input.into_iter()
        .map(|line| Round::from_str(line).unwrap())
        .collect();

    rounds.sort_by(|a, b| {
        a.hand.partial_cmp(&b.hand).unwrap()
    });

    rounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_07/input.txt").lines();

        let rounds: Vec<Round> = ordered_rounds(&input.collect());

        let sum = rounds.iter()
            .map(|round| round.bid)
            .enumerate()
            .map(|(i, bid)| ((i as u32 + 1) * bid) as u64)
            .sum::<u64>();

        assert_eq!(sum, 252656917);
    }

    #[test]
    fn example() {
        let input = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483\n\
        ".lines();

        let rounds: Vec<Round> = ordered_rounds(&input.collect());

        let sum = rounds.iter()
            .map(|round| round.bid)
            .enumerate()
            .map(|(i, bid)| ((i as u32 + 1) * bid) as u64)
            .sum::<u64>();

        assert_eq!(sum, 6440);
    }

    #[test]
    fn round_parsing() {
        let round = Round::from_str("32T3K 765").unwrap();
        assert_eq!(vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King], round.hand.0);
        assert_eq!(765, round.bid);
    }

    #[test]
    fn hand_types() {
        assert_eq!(HandType::FiveKind, Hand(vec![Card::Three, Card::Three, Card::Three, Card::Three, Card::Three]).hand_type());
        assert_eq!(HandType::FourKind, Hand(vec![Card::Three, Card::Three, Card::Three, Card::Three, Card::Four]).hand_type());
        assert_eq!(HandType::FullHouse, Hand(vec![Card::Three, Card::Three, Card::Three, Card::Two, Card::Two]).hand_type());
        assert_eq!(HandType::ThreeKind, Hand(vec![Card::Three, Card::Three, Card::Three, Card::Two, Card::Four]).hand_type());
        assert_eq!(HandType::TwoPair, Hand(vec![Card::Two, Card::Two, Card::Four, Card::Four, Card::Six]).hand_type());
        assert_eq!(HandType::OnePair, Hand(vec![Card::Two, Card::Two, Card::Four, Card::Five, Card::Six]).hand_type());
        assert_eq!(HandType::HighCard, Hand(vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]).hand_type());
    }

    #[test]
    fn hand_ordering() {
        let hand_1 = Hand(vec![Card::Three, Card::Three, Card::Three, Card::Three, Card::Three]);
        let hand_2 = Hand(vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]);

        assert!(hand_1 > hand_2);
    }
}
