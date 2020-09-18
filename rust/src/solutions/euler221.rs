use crate::utils::numeric::divisors_with_sieve::divisors;

pub fn euler221() -> String {
    let sieve = primal::Sieve::new(20_000_000);
    let mut vec = Vec::<u128>::new();
    for _p in 1..=100000 {
        for d in divisors(&sieve, _p * _p + 1).into_iter().map(|d| d as u128) {
            let p = _p as u128;
            vec.push(p * (p + d) * (p + (p * p + 1) / d));
        }
    }
    vec.sort();
    vec.dedup();
    vec[149999].to_string()
}
