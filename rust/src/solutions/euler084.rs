use rand::{
    distributions::{Distribution, Uniform},
    rngs::ThreadRng,
    seq::SliceRandom,
};
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, PartialEq, Eq)]
struct Square(usize);

const SQUARE_GO: Square = Square(0);
const SQUARE_R1: Square = Square(5);
const SQUARE_JAIL: Square = Square(10);
const SQUARE_C1: Square = Square(11);
const SQUARE_E3: Square = Square(24);
const SQUARE_G2J: Square = Square(30);
const SQUARE_H2: Square = Square(39);
const SQUARES_CC: [Square; 3] = [Square(2), Square(17), Square(33)];
const SQUARES_CH: [Square; 3] = [Square(7), Square(22), Square(36)];

fn next_r(&Square(n): &Square) -> Square {
    Square(match n {
        _ if n > 5 && n < 15 => 15,
        _ if n > 15 && n < 25 => 25,
        _ if n > 25 && n < 35 => 35,
        _ => 5,
    })
}

fn next_u(&Square(n): &Square) -> Square {
    Square(match n {
        _ if n > 12 && n < 28 => 28,
        _ => 12,
    })
}

impl Add<usize> for Square {
    type Output = Square;
    fn add(self, rhs: usize) -> Self::Output {
        let Square(n) = self;
        Square((n + rhs) % 40)
    }
}

impl AddAssign<usize> for Square {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Card {
    Go,
    Jail,
    C1,
    E3,
    H2,
    R1,
    NextR,
    NextU,
    Back3,
    Null,
}

struct GameState {
    position: Square,
    visited_counter: [usize; 40],
    doubles: u8,
    community_deck: Vec<Card>,
    chance_deck: Vec<Card>,
    gen: ThreadRng,
    dice: Uniform<usize>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut gen = rand::thread_rng();
        let mut community_deck = vec![Card::Go, Card::Jail];
        community_deck.extend(&[Card::Null; 14]);
        community_deck.shuffle(&mut gen);
        let mut chance_deck = vec![
            Card::Go,
            Card::Jail,
            Card::C1,
            Card::E3,
            Card::H2,
            Card::R1,
            Card::NextR,
            Card::NextR,
            Card::NextU,
            Card::Back3,
        ];
        chance_deck.extend(&[Card::Null; 6]);
        chance_deck.shuffle(&mut gen);
        GameState {
            position: SQUARE_GO,
            visited_counter: [0; 40],
            doubles: 0,
            community_deck,
            chance_deck,
            gen,
            dice: Uniform::from(1..5),
        }
    }

    fn roll_dice(&mut self) -> usize {
        self.dice.sample(&mut self.gen)
    }

    fn go_to_square(&mut self, &square: &Square) {
        let Square(n) = match square {
            SQUARE_G2J => SQUARE_JAIL,
            _ => square,
        };
        self.visited_counter[n] += 1;
        self.position = Square(n);
    }

    fn interpret_card(&mut self, &card: &Card) {
        match card {
            Card::Go => self.go_to_square(&SQUARE_GO),
            Card::Jail => self.go_to_square(&SQUARE_JAIL),
            Card::C1 => self.go_to_square(&SQUARE_C1),
            Card::E3 => self.go_to_square(&SQUARE_E3),
            Card::H2 => self.go_to_square(&SQUARE_H2),
            Card::R1 => self.go_to_square(&SQUARE_R1),
            Card::NextR => self.go_to_square(&next_r(&self.position)),
            Card::NextU => self.go_to_square(&next_u(&self.position)),
            Card::Back3 => self.go_to_square(&(self.position + 37)), // -3 == +37 mod 40
            Card::Null => self.go_to_square(&self.position.clone()),
        }
    }

    fn draw_community_card(&mut self) {
        self.interpret_card(&self.community_deck[0].clone());
        self.community_deck.rotate_left(1);
    }

    fn draw_chance_card(&mut self) -> () {
        self.interpret_card(&self.chance_deck[0].clone());
        self.chance_deck.rotate_left(1);
    }

    fn move_pawn(&mut self, new_position: Square) {
        match new_position {
            _ if SQUARES_CC.contains(&new_position) => {
                self.position = new_position;
                self.draw_community_card();
            }
            _ if SQUARES_CH.contains(&new_position) => {
                self.position = new_position;
                self.draw_chance_card();
            }
            _ => self.go_to_square(&new_position),
        }
    }

    fn get_next_position(&mut self) -> Square {
        let roll1: usize = self.roll_dice();
        let roll2: usize = self.roll_dice();
        self.doubles = if roll1 == roll2 { self.doubles + 1 } else { 0 };
        if self.doubles == 3 {
            self.doubles = 0;
            SQUARE_JAIL
        } else {
            self.position + (roll1 + roll2)
        }
    }

    pub fn simulate_game(&mut self, steps: usize) {
        for _ in 0..steps {
            let next_pos = self.get_next_position();
            self.move_pawn(next_pos);
        }
    }

    pub fn get_answer(&self) -> String {
        let mut answers: Vec<(usize, &usize)> =
            (0usize..).zip(self.visited_counter.iter()).collect();
        answers.sort_by_key(|n| n.1);
        format!(
            "{:02}{:02}{:02}",
            answers[39].0, answers[38].0, answers[37].0
        )
    }
}

pub fn euler084() -> String {
    let mut state = GameState::new();
    state.simulate_game(2000000);
    state.get_answer()
}
