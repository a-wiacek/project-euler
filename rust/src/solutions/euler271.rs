use crate::utils::numeric::modpow::ModPow;
use itertools::Itertools;
use std::ops::Sub;

/*
Factorizing 13082761331670030, we can see that it is product of all primes up to 43.
By chinese remainder theorem, this congruence is equal to following congruences:
 x^3 = 1 mod 2
 x^3 = 1 mod 3
 ...
 x^3 = 1 mod 43
*/

pub fn euler271() -> String {
    let primes = primal::Sieve::new(43)
        .primes_from(0)
        .take_while(|&p| p <= 43)
        .map(|p| p as u128)
        .collect::<Vec<_>>();
    let q: u128 = primes.iter().product();
    let inverses = primes
        .iter()
        .map(|&p| {
            let m = q / p;
            m * m.modpow(&(p - 2), &p)
        })
        .collect::<Vec<_>>();
    primes
        .into_iter()
        .map(|p| (1..p).filter(move |&x| (x * x * x) % p == 1))
        .multi_cartesian_product()
        .map(|sol_choice| {
            sol_choice
                .into_iter()
                .zip(&inverses)
                .map(|(x, &y)| x * y)
                .sum::<u128>()
                % q
        })
        .sum::<u128>()
        .sub(1)
        .to_string()
}
