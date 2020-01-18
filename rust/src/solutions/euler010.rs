pub fn euler010() -> String {
    let bound = 2000000;
    primal::Sieve::new(bound)
        .primes_from(0)
        .take_while(|x| x < &bound)
        .sum::<usize>()
        .to_string()
}