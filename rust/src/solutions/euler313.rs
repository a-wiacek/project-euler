use std::ops::Mul;

pub fn euler313() -> String {
    primal::Sieve::new(1000000)
        .primes_from(0)
        .map(|p| match p {
            2 => 0,
            3 => 1,
            _ => (p * p - 1) / 24,
        })
        .sum::<usize>()
        .mul(2)
        .to_string()
}
