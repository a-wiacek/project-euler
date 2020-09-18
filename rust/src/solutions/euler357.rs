use num::integer::Roots;
use std::ops::Add;

pub fn euler357() -> String {
    let bound = 100_000_000;
    let sieve = primal::Sieve::new(bound);
    (2..=bound)
        .step_by(4)
        .filter(|&n| {
            (1..=n.sqrt())
                .filter(|&d| n % d == 0)
                .map(|d| d + n / d)
                .all(|d| sieve.is_prime(d))
        })
        .sum::<usize>()
        .add(1)
        .to_string()
}
