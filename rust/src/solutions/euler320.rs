use crate::utils::numeric::digits::Digits;

fn primes_in_factorial(n: usize, p: usize) -> usize {
    (n - n.digits_in_radix(p).into_iter().sum::<usize>()) / (p - 1)
}

fn check_fac(mut n: usize, p: usize, k: usize) -> usize {
    while primes_in_factorial(n, p) < k {
        n = n - n % p + p;
    }
    n
}

fn inverse(p: usize, k: usize) -> usize {
    check_fac(k * (p - 1), p, k)
}

pub fn euler320() -> String {
    let bound = 1000000;
    let mut sum = 0;
    let mut max = 0;
    let sieve = primal::Sieve::new(bound);
    for n in 10..=bound {
        for (p, _) in sieve.factor(n).unwrap() {
            let count = primes_in_factorial(n, p);
            let inv = inverse(p, count * 1234567890);
            if inv > max {
                max = inv;
            }
        }
        sum += max;
        sum %= 10usize.pow(18);
    }
    sum.to_string()
}
