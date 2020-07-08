use crate::utils::numeric::digits::Digits;
use num::{cast::ToPrimitive, pow::Pow, BigUint};

pub fn euler119() -> String {
    let mut pows = Vec::new();
    let _10 = BigUint::from(10u32);
    for a in 1u32..=100 {
        for b in 1u32..=50 {
            let s = BigUint::from(a).pow(b);
            if &s > &_10 && s.digits().into_iter().sum::<BigUint>().to_u32().unwrap() == a {
                pows.push(s);
            }
        }
    }
    pows.sort();
    pows.dedup();
    pows[29].to_string()
}
