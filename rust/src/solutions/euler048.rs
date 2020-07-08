use num::{pow::Pow, BigInt};

pub fn euler048() -> String {
    ((1usize..=1000)
        .map(|n| BigInt::from(n).pow(n))
        .sum::<BigInt>()
        % BigInt::from(10000000000i64))
    .to_string()
}
