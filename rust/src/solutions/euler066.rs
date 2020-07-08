use crate::utils::numeric::sequences::continued_fractions::{approximations, square_root};
use num::{integer::Roots, pow::Pow, BigInt};

pub fn euler066() -> String {
    (2u32..1000)
        .filter_map(|n| {
            if n.sqrt().pow(2) == n {
                None
            } else {
                let sqrt = square_root::SquareRoot::<BigInt>::new(BigInt::from(n), true);
                let mut approxs = approximations::Approximations::new(sqrt);
                Some((
                    n,
                    approxs
                        .find(|approx| {
                            approx.numer().pow(&2u32) - BigInt::from(n) * approx.denom().pow(&2u32)
                                == BigInt::from(1)
                        })
                        .unwrap()
                        .numer()
                        .clone(),
                ))
            }
        })
        .max_by_key(|t| t.1.clone())
        .unwrap()
        .0
        .to_string()
}
