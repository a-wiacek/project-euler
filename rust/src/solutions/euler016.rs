use crate::utils::numeric::digits::Digits;
use num::bigint::BigInt;
use num::pow::Pow;

pub fn euler016() -> String {
    BigInt::from(2)
        .pow(1000usize)
        .digits()
        .into_iter()
        .sum::<BigInt>()
        .to_string()
}
