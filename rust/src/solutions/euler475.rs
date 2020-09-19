use crate::utils::numeric::modpow::ModPow;
use cached::proc_macro::cached;
use num::integer::binomial;

const P: u64 = 1_000_000_007;

#[cached]
fn compute(n1: u8, n2: u8, n3: u8, n4: u8) -> u64 {
    if n1 == 0 && n2 == 0 && n3 == 0 && n4 == 0 {
        return 1;
    }
    let mut ans = 0;
    for k1 in 0..=3 {
        for k2 in 0..=3 - k1 {
            for k3 in 0..=3 - k1 - k2 {
                let k4 = 3 - k1 - k2 - k3;
                if k1 <= n1 && k2 <= n2 && k3 <= n3 && k4 <= n4 {
                    let s = vec![
                        binomial(n1 as u128, k1 as u128) % P as u128,
                        binomial(n2 as u128, k2 as u128) % P as u128 * 2u128.pow(k2 as u32),
                        binomial(n3 as u128, k3 as u128) % P as u128 * 3u128.pow(k3 as u32),
                        binomial(n4 as u128, k4 as u128) % P as u128 * 4u128.pow(k4 as u32),
                    ]
                    .into_iter()
                    .fold(1, |a, b| a * b as u64 % P);
                    let re = compute(n1 - k1 + k2, n2 - k2 + k3, n3 - k3 + k4, n4 - k4);
                    ans += s * re;
                    ans %= P;
                }
            }
        }
    }
    ans
}

// n < 1024
fn solutions(n: u64) -> u64 {
    (1..=n / 3).fold(1, |a, b| a * b % P).modpow(&(P - 2), &P) * compute(0, 0, 0, (n / 4) as u8) % P
}

pub fn euler475() -> String {
    solutions(600).to_string()
}
