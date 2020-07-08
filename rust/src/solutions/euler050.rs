use primal::Sieve;

pub fn euler050() -> String {
    let bound = 1000000;
    let sieve = Sieve::new(bound);
    let primes: Vec<usize> = sieve.primes_from(0).collect();
    let mut best_len = 0;
    let mut best_prime = 0;
    for begin in 0..primes.len() {
        let mut end = begin + 1;
        let mut sum = primes[begin];
        while sum < bound && end < primes.len() {
            if sieve.is_prime(sum) {
                let l = end - begin;
                if l > best_len {
                    best_len = l;
                    best_prime = sum;
                }
            }
            sum += primes[end];
            end += 1;
        }
    }
    best_prime.to_string()
}
