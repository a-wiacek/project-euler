use crate::utils::numeric::factorial::factorial as fac;
use num::integer::binomial as bin;

fn count(n: u128) -> u128 {
    let mut ans = 0;
    // Digits appearing thrice
    for a in 0..=n / 3 {
        // Digits appearing twice
        for b in 0..=n / 2 {
            // Digits appearing once
            for c in 0..=n {
                if 3 * a + 2 * b + c == n && a + b + c <= 9 {
                    // Choose digits appearing x times * choose their alignment * choose their places
                    let n3 = bin(9, a) * bin(n, 3 * a) * fac(&(3 * a)) / fac(&3u128).pow(a as u32);
                    let n2 = bin(9 - a, b) * bin(n - 3 * a, 2 * b) * fac(&(2 * b))
                        / fac(&2u128).pow(b as u32);
                    let n1 = bin(9 - a - b, c) * fac(&c);
                    ans += n3 * n2 * n1;
                }
            }
        }
    }
    ans
}

pub fn euler172() -> String {
    (9 * (count(17) + 17 * count(16) + 17 * 8 * count(15))).to_string()
}
