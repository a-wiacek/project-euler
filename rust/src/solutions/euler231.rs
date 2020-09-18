fn primes_sum(n: usize, _k: usize) -> usize {
    let sieve = primal::Sieve::new(n);
    let k = if _k + _k > n { n - _k } else { _k };
    let f = |iter: std::ops::RangeInclusive<usize>| {
        iter.flat_map(|n| sieve.factor(n).unwrap())
            .map(|(p, exp)| p * exp)
            .sum::<usize>()
    };
    f(n - k + 1..=n) - f(1..=k)
}

pub fn euler231() -> String {
    primes_sum(20_000_000, 15_000_000).to_string()
}
