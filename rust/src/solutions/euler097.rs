use num::{pow::Pow, BigInt};

pub fn euler097() -> String {
    ((BigInt::from(28433) * BigInt::from(2).pow(7830457usize) + BigInt::from(1))
        % BigInt::from(10000000000i64))
    .to_string()
}
