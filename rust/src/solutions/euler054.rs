use itertools::Itertools;
use std::cmp::Ordering;

type Value = usize;
const ACE: Value = 14;
const KING: Value = 13;
const QUEEN: Value = 12;
const JACK: Value = 11;

fn parse_value(c: char) -> Value {
    match c {
        'A' => ACE,
        'K' => KING,
        'Q' => QUEEN,
        'J' => JACK,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as usize,
    }
}

#[derive(Eq, PartialEq)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

fn parse_suit(c: char) -> Suit {
    match c {
        'D' => Suit::Diamond,
        'C' => Suit::Club,
        'H' => Suit::Heart,
        _ => Suit::Spade,
    }
}

#[derive(Eq, PartialEq)]
struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_card(s: &[char]) -> Card {
    Card {
        value: parse_value(s[0]),
        suit: parse_suit(s[1]),
    }
}

struct Hand([Card; 5]);

fn parse_hand(s: &[char]) -> Hand {
    let mut cards = [
        parse_card(&s[0..2]),
        parse_card(&s[3..5]),
        parse_card(&s[6..8]),
        parse_card(&s[9..11]),
        parse_card(&s[12..14]),
    ];
    cards.sort();
    cards.reverse();
    Hand(cards)
}

fn parse_hands(s: &[char]) -> (Hand, Hand) {
    (parse_hand(&s[..15]), parse_hand(&s[15..]))
}

// Card fields represent remaining (not used in) cards
// Those cards are sorted with respect to their value

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct HighCard {
    pub remaining: [Value; 5],
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct OnePair {
    pub pair: Value,
    pub remaining: [Value; 3],
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct TwoPairs {
    pub higher_pair: Value,
    pub lower_pair: Value,
    pub remaining: Value,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Three {
    pub three: Value,
    pub remaining: [Value; 2],
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Straight {
    pub highest: Value,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Flush {
    pub remaining: [Value; 5],
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct FullHouse {
    pub three: Value,
    pub two: Value,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Four {
    pub four: Value,
    pub remaining: Value,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct StraightFlush {
    pub highest: Value,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum HandRank {
    HighCard(HighCard),
    OnePair(OnePair),
    TwoPairs(TwoPairs), // value1 > value2
    Three(Three),
    Straight(Straight), // highest value
    Flush(Flush),
    FullHouse(FullHouse), // value1 x 3 + value2 x 2
    Four(Four),
    StraightFlush(StraightFlush), // highest value
    RoyalFlush,
}

fn find_straight(Hand(cards): &Hand) -> Option<Straight> {
    let values: Vec<Value> = cards.iter().map(|c| c.value).collect();
    if (1..5).all(|i| values[i] == values[0] - i) {
        Some(Straight { highest: values[0] })
    } else {
        None
    }
}

fn find_flush(Hand(cards): &Hand) -> Option<Flush> {
    if cards.iter().map(|c| c.value).all_equal() {
        Some(Flush {
            remaining: [
                cards[0].value,
                cards[1].value,
                cards[2].value,
                cards[3].value,
                cards[4].value,
            ],
        })
    } else {
        None
    }
}

fn find_four(Hand(cards): &Hand) -> Option<Four> {
    if cards[0..4].iter().map(|c| c.value).all_equal() {
        Some(Four {
            four: cards[2].value,
            remaining: cards[4].value,
        })
    } else if cards[1..5].iter().map(|c| c.value).all_equal() {
        Some(Four {
            four: cards[2].value,
            remaining: cards[0].value,
        })
    } else {
        None
    }
}

fn rank_low_hand(Hand([card1, card2, card3, card4, card5]): &Hand) -> HandRank {
    let v = card3.value;
    if [v, card1.value, card2.value].iter().all_equal() {
        if card4.value == card5.value {
            HandRank::FullHouse(FullHouse {
                three: v,
                two: card4.value,
            })
        } else {
            HandRank::Three(Three {
                three: v,
                remaining: [card4.value, card5.value],
            })
        }
    } else if [v, card4.value, card2.value].iter().all_equal() {
        HandRank::Three(Three {
            three: v,
            remaining: [card1.value, card5.value],
        })
    } else if [v, card4.value, card5.value].iter().all_equal() {
        if card1.value == card2.value {
            HandRank::FullHouse(FullHouse {
                three: v,
                two: card1.value,
            })
        } else {
            HandRank::Three(Three {
                three: v,
                remaining: [card1.value, card2.value],
            })
        }
    } else {
        let p1 = card1.value == card2.value;
        let p2 = card2.value == card3.value;
        let p3 = card3.value == card4.value;
        let p4 = card4.value == card5.value;
        match (p1, p2, p3, p4) {
            (true, false, true, false) => HandRank::TwoPairs(TwoPairs {
                higher_pair: card1.value,
                lower_pair: card3.value,
                remaining: card5.value,
            }),
            (false, true, false, true) => HandRank::TwoPairs(TwoPairs {
                higher_pair: card2.value,
                lower_pair: card4.value,
                remaining: card1.value,
            }),
            (true, false, false, true) => HandRank::TwoPairs(TwoPairs {
                higher_pair: card1.value,
                lower_pair: card4.value,
                remaining: card3.value,
            }),
            (true, false, false, false) => HandRank::OnePair(OnePair {
                pair: card1.value,
                remaining: [card3.value, card4.value, card5.value],
            }),
            (false, true, false, false) => HandRank::OnePair(OnePair {
                pair: card2.value,
                remaining: [card1.value, card4.value, card5.value],
            }),
            (false, false, true, false) => HandRank::OnePair(OnePair {
                pair: card3.value,
                remaining: [card1.value, card2.value, card5.value],
            }),
            (false, false, false, true) => HandRank::OnePair(OnePair {
                pair: card4.value,
                remaining: [card1.value, card2.value, card3.value],
            }),
            _ => HandRank::HighCard(HighCard {
                remaining: [
                    card1.value,
                    card2.value,
                    card3.value,
                    card4.value,
                    card5.value,
                ],
            }),
        }
    }
}

fn rank_hand(hand: &Hand) -> HandRank {
    let opt_straight = find_straight(hand);
    let opt_flush = find_flush(hand);
    match opt_flush {
        Some(flush) => match opt_straight {
            Some(straight) => match straight.highest {
                ACE => HandRank::RoyalFlush,
                v => HandRank::StraightFlush(StraightFlush { highest: v }),
            },
            None => HandRank::Flush(flush),
        },
        None => match opt_straight {
            Some(straight) => HandRank::Straight(straight),
            None => match find_four(hand) {
                Some(four) => HandRank::Four(four),
                None => rank_low_hand(hand),
            },
        },
    }
}

fn eval_line(line: &str) -> (HandRank, HandRank) {
    let (hx, hy) = parse_hands(&line.chars().collect_vec());
    (rank_hand(&hx), rank_hand(&hy))
}

pub fn euler054() -> String {
    crate::utils::input::get_input(54)
        .lines()
        .map(eval_line)
        .filter(|(ra, rb)| ra > rb)
        .count()
        .to_string()
}
