use crate::utils::numeric::digits::Digits;
use crate::utils::numeric::sequences::continued_fractions::{approximations, e};
use num::BigInt;

pub fn euler065() -> String {
    approximations::Approximations::new(e::E::<BigInt>::new())
        .nth(99)
        .unwrap()
        .numer()
        .digits()
        .iter()
        .sum::<BigInt>()
        .to_string()
}
