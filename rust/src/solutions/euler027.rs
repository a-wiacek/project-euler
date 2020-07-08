use itertools::Itertools;
use primal::Sieve;

fn find_length(a: i32, b: i32, sieve: &Sieve) -> usize {
    (0..)
        .map(|n| n * n + a * n + b)
        .position(|k| k <= 1 || !sieve.is_prime(k as usize))
        .unwrap()
}

pub fn euler027() -> String {
    let sieve = Sieve::new(2000000);
    let (a_best, b_best) = (-999..1000)
        .cartesian_product(-1000..1001)
        .max_by_key(|(a, b)| find_length(*a, *b, &sieve))
        .unwrap();
    (a_best * b_best).to_string()
}
