use crate::utils::number_theory::invert_mod::InvertMod;
use crate::utils::numeric::{factorization::Factorization, modpow::ModPow};
use primal::Sieve;

fn f(e: i64, m: i64) -> i64 {
    10.modpow(&e, &m)
}

fn prim_root(sieve: &Sieve, n: i64) -> bool {
    let phi = n - 1;
    Factorization::new(phi as usize, sieve)
        .factors()
        .into_iter()
        .all(|(p, _)| f(phi / p as i64, n) != 1)
}

fn proper_end(n: i64) -> bool {
    let d = 100_000;
    let g = |x| {
        let y = x % d;
        if y < 0 {
            y + d
        } else {
            y
        }
    };
    g((f(n - 1, d) - 1) * n.invert_mod(&d).unwrap()) == 56789
}

fn cyclic_digits_sum(n: i64) -> i64 {
    let mut acc = 0;
    let mut rem = 9;
    loop {
        acc += rem / n;
        let r = rem % n;
        if r == 0 {
            break acc;
        } else {
            rem = 10 * r + 9;
        }
    }
}

pub fn euler358() -> String {
    let sieve = Sieve::new(100_000);
    // bounds are for 00000000137 part, found by plugging 10^11/137 and 10^11/138 to WolframAlpha
    let unique_prime = (724_637_680..=729_927_080)
        .filter(|&p| primal::is_prime(p))
        .map(|p| p as i64)
        .find(|&p| prim_root(&sieve, p) && proper_end(p))
        .unwrap();
    cyclic_digits_sum(unique_prime).to_string()
}
