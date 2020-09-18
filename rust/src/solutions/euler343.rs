use cached::{proc_macro::cached, UnboundCache};
use primal::Sieve;

fn is_prime(sieve: &Sieve, p: usize) -> bool {
    if p <= sieve.upper_bound() {
        sieve.is_prime(p)
    } else {
        primal::is_prime(p as u64)
    }
}

#[cached(
    type = "UnboundCache<usize, usize>",
    create = "{ UnboundCache::new() }",
    convert = "{ d }"
)]
fn compute(sieve: &Sieve, d: usize) -> usize {
    let e = d + 1;
    if is_prime(sieve, e) {
        d
    } else {
        let p = e / sieve.primes_from(0).find(|&x| e % x == 0).unwrap() - 1;
        compute(sieve, p)
    }
}

pub fn euler343() -> String {
    let sieve = Sieve::new(10_000_000);
    (1usize..=2000000)
        .map(|x| compute(&sieve, x.pow(3)))
        .sum::<usize>()
        .to_string()
}
