use num::bigint::BigInt;
use num::pow::Pow;

pub fn euler029() -> String {
    let mut vec: Vec<BigInt> = Vec::new();
    for a in 2..=100 {
        for b in 2usize..=100 {
            vec.push(BigInt::from(a).pow(b));
        }
    }
    vec.sort();
    vec.dedup();
    vec.len().to_string()
}
