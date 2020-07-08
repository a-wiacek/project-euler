use primal::Sieve;

fn prime_sum(n: usize, primes: &[usize]) -> usize {
    let p = primes[0];
    if n == 0 {
        1
    } else if n < p {
        0
    } else {
        prime_sum(n - p, primes) + prime_sum(n, &primes[1..])
    }
}

pub fn euler077() -> String {
    let primes = Sieve::new(100).primes_from(0).collect::<Vec<usize>>();
    (2..)
        .find(|&n| prime_sum(n, &primes[0..]) > 5000)
        .unwrap()
        .to_string()
}
