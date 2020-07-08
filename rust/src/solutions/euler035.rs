use crate::utils::numeric::digits::{undigits, Digits};
use primal::Sieve;

fn rotations(n: usize) -> Vec<usize> {
    let mut digits = n.digits();
    let mut rots = Vec::new();
    for _ in 0..digits.len() {
        rots.push(undigits(&digits));
        digits.rotate_left(1);
    }
    rots
}

pub fn euler035() -> String {
    let sieve = Sieve::new(1000000);
    sieve
        .primes_from(0)
        .take_while(|&p| p < 1000000)
        .filter(|&p| rotations(p).into_iter().all(|pp| sieve.is_prime(pp)))
        .count()
        .to_string()
}
