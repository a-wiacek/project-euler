pub fn euler187() -> String {
    let bound = 100_000_000;
    let sieve = primal::Sieve::new(bound);
    sieve
        .primes_from(0)
        .take_while(|&p| p < bound / 2)
        .map(|p| {
            sieve
                .primes_from(0)
                .take_while(|&pp| pp <= p && pp * p < bound)
                .count()
        })
        .sum::<usize>()
        .to_string()
}
