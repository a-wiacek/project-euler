use crate::utils::numeric::sequences::fibonacci::Fibonacci;
use num::bigint::BigInt;
use num::pow::Pow;

pub fn euler025() -> String {
    let bound = BigInt::from(10).pow(999usize);
    Fibonacci::new(BigInt::from(0), BigInt::from(1))
        .take_while(|n| n < &bound)
        .count()
        .to_string()
}
