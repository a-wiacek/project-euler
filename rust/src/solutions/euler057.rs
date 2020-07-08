use crate::utils::numeric::digits::Digits;
use crate::utils::numeric::sequences::continued_fractions::{approximations, square_root};
use num::BigInt;

pub fn euler057() -> String {
    approximations::Approximations::new(square_root::SquareRoot::<BigInt>::new(
        BigInt::from(2),
        true,
    ))
    .take(1000)
    .filter(|approx| approx.numer().digits().len() > approx.denom().digits().len())
    .count()
    .to_string()
}
