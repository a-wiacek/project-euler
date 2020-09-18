use crate::utils::number_theory::invert_mod::InvertMod;
use crate::utils::numeric::modpow::ModPow;

fn sum_m(n: i128) -> i128 {
    let n = n + 1;
    let m = 7i128.pow(9);
    let f = |x: i128| x.modpow(&n, &m);
    let inv_2 = 2.invert_mod(&m).unwrap();
    let inv_3 = 3.invert_mod(&m).unwrap();
    (2 * (f(2) - 1) - (f(3) - 1) * inv_2 + (f(4) - 1) * inv_3) % m
}

pub fn euler335() -> String {
    sum_m(10i128.pow(18)).to_string()
}
