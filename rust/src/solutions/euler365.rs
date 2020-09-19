use crate::utils::number_theory::chinese_remainder::chinese_remainder;
use num::{integer::binomial, BigInt, ToPrimitive};
use primal::Sieve;

fn binom_mod(mut n: BigInt, mut k: BigInt, p: &BigInt) -> BigInt {
    let mut ans = BigInt::from(1);
    let _0 = BigInt::from(0);
    while &k > &_0 && &n > &_0 {
        ans = ans * binomial(&n % p, &k % p) % p;
        n = n / p;
        k = k / p;
    }
    ans
}

pub fn euler365() -> String {
    let primes: Vec<usize> = Sieve::new(5000)
        .primes_from(1000)
        .take_while(|p| p < &5000)
        .collect();
    let mut mod1p: Vec<i64> = vec![0; 5000];
    let mut ans: i64 = 0;
    for &p in &primes {
        mod1p[p] = binom_mod(
            BigInt::from(10u64.pow(18)),
            BigInt::from(10u64.pow(9)),
            &BigInt::from(p),
        )
        .to_i64()
        .unwrap();
    }
    for &p in &primes {
        for &q in primes.iter().skip_while(|&&q| q <= p) {
            for &r in primes.iter().skip_while(|&&r| r <= q) {
                ans += chinese_remainder(&[
                    (mod1p[p], p as i64),
                    (mod1p[q], q as i64),
                    (mod1p[r], r as i64),
                ])
                .unwrap();
            }
        }
    }
    ans.to_string()
}
