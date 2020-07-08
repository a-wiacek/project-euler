use crate::utils::numeric::digits::{undigits, Digits};
use primal::Sieve;

fn truncations(n: usize) -> Vec<usize> {
    // n is assumed to be prime, so it's not included in truncs
    let digits = n.digits();
    let mut truncs = Vec::new();
    for i in 1..digits.len() {
        truncs.push(undigits(&digits[i..].into()));
        truncs.push(undigits(&digits[..i].into()));
    }
    truncs
}

pub fn euler037() -> String {
    let sieve = Sieve::new(1000000);
    sieve
        .primes_from(10)
        .take_while(|&p| p < 1000000)
        .filter(|&p| truncations(p).into_iter().all(|pp| sieve.is_prime(pp)))
        .sum::<usize>()
        .to_string()
}
