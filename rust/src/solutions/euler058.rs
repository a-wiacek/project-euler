use primal::Sieve;

pub fn euler058() -> String {
    let sieve = Sieve::new(1000000000);
    let mut primes = 0;
    let mut total = 1;
    let mut step = 2;
    let mut curr = 1;
    loop {
        for _ in 0..4 {
            curr += step;
            total += 1;
            if sieve.is_prime(curr) {
                primes += 1;
            }
        }
        if 10 * primes < total {
            break (step + 1).to_string();
        } else {
            step += 2;
        }
    }
}
