use crate::utils::numeric::modpow::ModPow;

pub fn euler132() -> String {
    primal::Sieve::new(1_000_000)
        .primes_from(0)
        .filter(|&p| 10usize.modpow(&1_000_000_000, &(9 * p)) == 1)
        .take(40)
        .sum::<usize>()
        .to_string()
}
