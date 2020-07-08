pub fn euler123() -> String {
    (1..)
        .zip(primal::Sieve::new(10000000).primes_from(0))
        .step_by(2)
        .find(|&(n, p)| 2 * n * p > 10_000_000_000)
        .unwrap()
        .0
        .to_string()
}
