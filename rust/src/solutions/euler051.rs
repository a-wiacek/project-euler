use crate::utils::numeric::digits::{undigits, Digits};
use itertools::Itertools;
use primal::Sieve;

fn has_eight_family(sieve: &Sieve, p: usize) -> bool {
    let p_digits = p.digits();
    let l = p_digits.len();
    let (positions_with_zero, positions_without_zero): (Vec<Vec<usize>>, Vec<Vec<usize>>) = (1..=l)
        .flat_map(|k| (0..l).combinations(k))
        .filter(|pos| pos.iter().map(|&i| p_digits[i]).all_equal())
        .partition(|pos| pos.contains(&0));
    let replace = |positions, digit| {
        let mut digits = p_digits.clone();
        for &pos in positions {
            digits[pos] = digit;
        }
        sieve.is_prime(undigits(&digits))
    };
    positions_with_zero
        .iter()
        .any(|pos| (1..10).filter(|&d| replace(pos, d)).count() >= 8)
        || positions_without_zero
            .iter()
            .any(|pos| (0..10).filter(|&d| replace(pos, d)).count() >= 8)
}

pub fn euler051() -> String {
    let bound = 1000000;
    let sieve = Sieve::new(bound);
    sieve
        .primes_from(0)
        .find(|&p| has_eight_family(&sieve, p))
        .unwrap()
        .to_string()
}
