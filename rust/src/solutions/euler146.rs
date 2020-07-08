use primal::is_prime;

fn check(&n: &u64) -> bool {
    let f = |x| n * n + x;
    [1, 3, 7, 9, 13, 27].iter().all(|&x| is_prime(f(x)))
        && ![11, 17, 19, 21, 23].iter().any(|&x| is_prime(f(x)))
}

pub fn euler146() -> String {
    (10..150000000)
        .step_by(10)
        .filter(check)
        .sum::<u64>()
        .to_string()
}
