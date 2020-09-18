use crate::utils::number_theory::{chinese_remainder::chinese, invert_mod::InvertMod};
use crate::utils::numeric::modpow::ModPow;

pub fn euler312() -> String {
    let get_b = |n: i64, m: i64| (18.invert_mod(&m).unwrap() * (n - 27)) % m;
    let p = |e| 13i64.pow(e);
    let d0 = 10_000 % (13 * 13 * 12);
    let b0 = get_b(3.modpow(&d0, &p(3)), p(3));
    let ac0 = chinese(b0, p(3), 3, 12).unwrap();
    let c1 = 8 * 12.modpow(&ac0, &p(4)) % p(4);
    let d1 = chinese(c1, p(4), 0, 12).unwrap();
    let b1 = get_b(3.modpow(&d1, &p(5)), p(5));
    let ac1 = chinese(b1, p(5), 3, 12).unwrap();
    let c2 = 8 * 12.modpow(&ac1, &p(6)) % p(6);
    let d2 = chinese(c2, p(6), 0, 12).unwrap();
    let b2 = get_b(3.modpow(&d2, &p(7)), p(7));
    let ac2 = chinese(b2, p(7), 3, 12).unwrap();
    let c3 = 8 * 12.modpow(&ac2, &p(8)) % p(8);
    c3.to_string()
}
