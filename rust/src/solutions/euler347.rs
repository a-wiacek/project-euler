use primal::Sieve;
use std::cmp::max;
use std::iter::successors;

fn m(p: usize, q: usize, n: usize) -> usize {
    let init_list = successors(Some(p), |x| x.checked_mul(p))
        .map(|x| x * q)
        .take_while(|x| x <= &n)
        .collect::<Vec<_>>();
    match init_list.last() {
        None => 0,
        Some(r) => {
            let mut ans = 0;
            let mut r = *r;
            loop {
                if q * r > n {
                    if r % p == 0 {
                        ans = max(ans, r);
                        r /= p;
                    } else {
                        break ans;
                    }
                } else {
                    r *= q;
                }
            }
        }
    }
}

fn s(n: usize) -> usize {
    let sieve = Sieve::new(n);
    let primes = sieve.primes_from(0).collect::<Vec<_>>();
    let mut all = vec![];
    for &p in primes.iter().take_while(|&&p| p * p <= n) {
        for &q in primes
            .iter()
            .skip_while(|&&q| q <= p)
            .take_while(|&&q| p * q <= n)
        {
            all.push(m(p, q, n));
        }
    }
    all.sort();
    all.dedup();
    all.into_iter().sum()
}

pub fn euler347() -> String {
    s(10_000_000).to_string()
}
