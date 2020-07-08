use primal::is_prime;

pub fn euler291() -> String {
    (1u64..)
        .map(|n| n * n + (n + 1) * (n + 1))
        .take_while(|&n| n < 5 * 10u64.pow(15))
        .filter(|&n| is_prime(n))
        .count()
        .to_string()
}
