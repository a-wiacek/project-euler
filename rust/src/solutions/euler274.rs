use crate::utils::number_theory::invert_mod::InvertMod;

pub fn euler274() -> String {
    let bound = 10_000_000;
    primal::Sieve::new(bound)
        .primes_from(0)
        .take_while(|&n| n < bound)
        .filter_map(|p| 10i64.invert_mod(&(p as i64)))
        .sum::<i64>()
        .to_string()
}
