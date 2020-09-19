use crate::utils::numeric::sequences::fibonacci::Fibonacci;
use std::cmp::Ordering;

const P: usize = 1_000_000_000;

pub fn euler618() -> String {
    let fibs = Fibonacci::new(1, 2).take(23).collect::<Vec<usize>>();
    let last_fib = *fibs.last().unwrap();
    let sieve = primal::Sieve::new(last_fib + 100);
    let p_max = (last_fib..).find(|&p| sieve.is_prime(p)).unwrap();
    let primes = sieve
        .primes_from(0)
        .take_while(|&p| p <= p_max)
        .collect::<Vec<_>>();
    let mut s = vec![vec![0; p_max + 1]; primes.len()];
    for i in (0..primes.len()).rev() {
        for sum_left in 0..=p_max {
            let p = primes[i];
            s[i][sum_left] = match sum_left.cmp(&p) {
                Ordering::Less => 0,
                Ordering::Equal => p,
                Ordering::Greater => (p * s[i][sum_left - p] + s[i + 1][sum_left]) % P,
            };
        }
    }
    let ans = fibs.into_iter().map(|fib| s[0][fib]).sum::<usize>() % P;
    ans.to_string()
}
