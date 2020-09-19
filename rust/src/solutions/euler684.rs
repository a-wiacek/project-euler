use crate::utils::numeric::{modpow::ModPow, sequences::fibonacci::Fibonacci};

pub fn euler684() -> String {
    let p = 1_000_000_007;
    let s = |n| {
        let k = (n - 1) / 9;
        let l = n - 9 * k;
        10.modpow(&k, &p) * (6 + l * (l + 3) / 2) - 6 - 9 * k - l
    };
    let s = Fibonacci::<i64>::new(1, 2).take(89).map(s).sum::<i64>() % p;
    if s < 0 {
        (s + p).to_string()
    } else {
        s.to_string()
    }
}
